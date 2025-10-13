"""
SpiraPi Transaction Management System
Handles ACID transactions, rollbacks, and data consistency
"""

from enum import Enum
from typing import Dict, List, Any, Optional, Set, Callable
from dataclasses import dataclass, field
from datetime import datetime
import uuid
import json
import threading
from contextlib import contextmanager


class TransactionState(Enum):
    """Transaction states"""
    ACTIVE = "active"
    COMMITTED = "committed"
    ROLLED_BACK = "rolled_back"
    ABORTED = "aborted"


class IsolationLevel(Enum):
    """Transaction isolation levels"""
    READ_UNCOMMITTED = "read_uncommitted"
    READ_COMMITTED = "read_committed"
    REPEATABLE_READ = "repeatable_read"
    SERIALIZABLE = "serializable"


@dataclass
class TransactionLog:
    """Log entry for a transaction operation"""
    operation_id: str
    operation_type: str  # INSERT, UPDATE, DELETE, CREATE_TABLE, etc.
    table_name: str
    record_id: str
    old_data: Any = None
    new_data: Any = None
    timestamp: datetime = field(default_factory=datetime.now)
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert log entry to dictionary"""
        return {
            "operation_id": self.operation_id,
            "operation_type": self.operation_type,
            "table_name": self.table_name,
            "record_id": self.record_id,
            "old_data": self.old_data,
            "new_data": self.new_data,
            "timestamp": self.timestamp.isoformat()
        }
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'TransactionLog':
        """Create log entry from dictionary"""
        data = data.copy()
        data["timestamp"] = datetime.fromisoformat(data["timestamp"])
        return cls(**data)


@dataclass
class Transaction:
    """Represents a database transaction"""
    transaction_id: str
    state: TransactionState = TransactionState.ACTIVE
    isolation_level: IsolationLevel = IsolationLevel.READ_COMMITTED
    created_at: datetime = field(default_factory=datetime.now)
    committed_at: Optional[datetime] = None
    rollback_reason: Optional[str] = None
    
    # Transaction data
    operations: List[TransactionLog] = field(default_factory=list)
    locks_held: Set[str] = field(default_factory=set)
    savepoints: Dict[str, int] = field(default_factory=dict)
    
    def add_operation(self, operation: TransactionLog) -> None:
        """Add an operation to the transaction"""
        self.operations.append(operation)
    
    def create_savepoint(self, name: str) -> None:
        """Create a savepoint"""
        self.savepoints[name] = len(self.operations)
    
    def rollback_to_savepoint(self, name: str) -> None:
        """Rollback to a specific savepoint"""
        if name not in self.savepoints:
            raise ValueError(f"Savepoint '{name}' not found")
        
        savepoint_index = self.savepoints[name]
        self.operations = self.operations[:savepoint_index]
        
        # Remove savepoints after this one
        self.savepoints = {k: v for k, v in self.savepoints.items() if v <= savepoint_index}
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert transaction to dictionary"""
        return {
            "transaction_id": self.transaction_id,
            "state": self.state.value,
            "isolation_level": self.isolation_level.value,
            "created_at": self.created_at.isoformat(),
            "committed_at": self.committed_at.isoformat() if self.committed_at else None,
            "rollback_reason": self.rollback_reason,
            "operations_count": len(self.operations),
            "locks_held": list(self.locks_held),
            "savepoints": self.savepoints
        }


class LockManager:
    """Manages database locks for transaction isolation"""
    
    def __init__(self):
        self.locks: Dict[str, Dict[str, Transaction]] = {}  # resource -> {lock_type -> transaction}
        self.waiting_transactions: Dict[str, List[Transaction]] = {}  # resource -> waiting transactions
        self.lock_timeout = 30  # seconds
    
    def acquire_lock(self, transaction: Transaction, resource: str, 
                    lock_type: str = "SHARED", timeout: int = None) -> bool:
        """Acquire a lock on a resource"""
        if timeout is None:
            timeout = self.lock_timeout
        
        if resource not in self.locks:
            self.locks[resource] = {}
            self.waiting_transactions[resource] = []
        
        # Check if lock can be acquired
        if self._can_acquire_lock(transaction, resource, lock_type):
            self.locks[resource][lock_type] = transaction
            transaction.locks_held.add(f"{resource}:{lock_type}")
            return True
        
        # Add to waiting queue
        if transaction not in self.waiting_transactions[resource]:
            self.waiting_transactions[resource].append(transaction)
        
        # Wait for lock (simplified implementation)
        start_time = datetime.now()
        while (datetime.now() - start_time).seconds < timeout:
            if self._can_acquire_lock(transaction, resource, lock_type):
                self.locks[resource][lock_type] = transaction
                transaction.locks_held.add(f"{resource}:{lock_type}")
                self.waiting_transactions[resource].remove(transaction)
                return True
        
        # Timeout reached
        self.waiting_transactions[resource].remove(transaction)
        return False
    
    def release_lock(self, transaction: Transaction, resource: str, lock_type: str = "SHARED") -> None:
        """Release a lock on a resource"""
        lock_key = f"{resource}:{lock_type}"
        if lock_key in transaction.locks_held:
            transaction.locks_held.remove(lock_key)
            
            if resource in self.locks and lock_type in self.locks[resource]:
                if self.locks[resource][lock_type] == transaction:
                    del self.locks[resource][lock_type]
    
    def release_all_locks(self, transaction: Transaction) -> None:
        """Release all locks held by a transaction"""
        locks_to_release = list(transaction.locks_held)
        for lock_key in locks_to_release:
            resource, lock_type = lock_key.split(":", 1)
            self.release_lock(transaction, resource, lock_type)
    
    def _can_acquire_lock(self, transaction: Transaction, resource: str, lock_type: str) -> bool:
        """Check if a lock can be acquired"""
        if resource not in self.locks:
            return True
        
        current_locks = self.locks[resource]
        
        # SHARED locks can coexist with other SHARED locks
        if lock_type == "SHARED":
            return "EXCLUSIVE" not in current_locks
        
        # EXCLUSIVE locks require no other locks
        if lock_type == "EXCLUSIVE":
            return len(current_locks) == 0
        
        return True


class TransactionManager:
    """Manages all database transactions"""
    
    def __init__(self, storage_path: str = "data"):
        self.storage_path = storage_path
        self.active_transactions: Dict[str, Transaction] = {}
        self.committed_transactions: Dict[str, Transaction] = {}
        self.lock_manager = LockManager()
        self.transaction_counter = 0
        self._lock = threading.Lock()
        
        # Load transaction history
        self._load_transaction_history()
    
    def begin_transaction(self, isolation_level: IsolationLevel = IsolationLevel.READ_COMMITTED) -> str:
        """Begin a new transaction"""
        with self._lock:
            transaction_id = f"tx_{self.transaction_counter:08d}_{uuid.uuid4().hex[:8]}"
            self.transaction_counter += 1
            
            transaction = Transaction(
                transaction_id=transaction_id,
                isolation_level=isolation_level
            )
            
            self.active_transactions[transaction_id] = transaction
            return transaction_id
    
    def commit_transaction(self, transaction_id: str) -> bool:
        """Commit a transaction"""
        with self._lock:
            if transaction_id not in self.active_transactions:
                raise ValueError(f"Transaction '{transaction_id}' not found or already committed")
            
            transaction = self.active_transactions[transaction_id]
            
            try:
                # Validate transaction
                if not self._validate_transaction(transaction):
                    return False
                
                # Apply all operations
                self._apply_transaction(transaction)
                
                # Update transaction state
                transaction.state = TransactionState.COMMITTED
                transaction.committed_at = datetime.now()
                
                # Move to committed transactions
                self.committed_transactions[transaction_id] = transaction
                del self.active_transactions[transaction_id]
                
                # Release all locks
                self.lock_manager.release_all_locks(transaction)
                
                # Save transaction history
                self._save_transaction_history()
                
                return True
                
            except Exception as e:
                # Rollback on error
                self.rollback_transaction(transaction_id, str(e))
                return False
    
    def rollback_transaction(self, transaction_id: str, reason: str = "User requested rollback") -> bool:
        """Rollback a transaction"""
        with self._lock:
            if transaction_id not in self.active_transactions:
                raise ValueError(f"Transaction '{transaction_id}' not found or already committed")
            
            transaction = self.active_transactions[transaction_id]
            
            # Update transaction state
            transaction.state = TransactionState.ROLLED_BACK
            transaction.rollback_reason = reason
            
            # Release all locks
            self.lock_manager.release_all_locks(transaction)
            
            # Remove from active transactions
            del self.active_transactions[transaction_id]
            
            # Save transaction history
            self._save_transaction_history()
            
            return True
    
    def get_transaction(self, transaction_id: str) -> Optional[Transaction]:
        """Get a transaction by ID"""
        return self.active_transactions.get(transaction_id) or self.committed_transactions.get(transaction_id)
    
    def is_transaction_active(self, transaction_id: str) -> bool:
        """Check if a transaction is active"""
        return transaction_id in self.active_transactions
    
    def get_active_transactions(self) -> List[Transaction]:
        """Get all active transactions"""
        return list(self.active_transactions.values())
    
    def add_operation(self, transaction_id: str, operation: TransactionLog) -> bool:
        """Add an operation to a transaction"""
        if transaction_id not in self.active_transactions:
            return False
        
        transaction = self.active_transactions[transaction_id]
        transaction.add_operation(operation)
        return True
    
    def create_savepoint(self, transaction_id: str, name: str) -> bool:
        """Create a savepoint in a transaction"""
        if transaction_id not in self.active_transactions:
            return False
        
        transaction = self.active_transactions[transaction_id]
        transaction.create_savepoint(name)
        return True
    
    def rollback_to_savepoint(self, transaction_id: str, name: str) -> bool:
        """Rollback to a savepoint in a transaction"""
        if transaction_id not in self.active_transactions:
            return False
        
        transaction = self.active_transactions[transaction_id]
        try:
            transaction.rollback_to_savepoint(name)
            return True
        except ValueError:
            return False
    
    def _validate_transaction(self, transaction: Transaction) -> bool:
        """Validate a transaction before committing"""
        # Check if all operations are valid
        for operation in transaction.operations:
            if not self._validate_operation(operation):
                return False
        return True
    
    def _validate_operation(self, operation: TransactionLog) -> bool:
        """Validate a single operation"""
        # This is a simplified validation
        # In a real system, this would check constraints, permissions, etc.
        return True
    
    def _apply_transaction(self, transaction: Transaction) -> None:
        """Apply all operations in a transaction"""
        # This is a simplified implementation
        # In a real system, this would apply changes to the actual storage
        for operation in transaction.operations:
            self._apply_operation(operation)
    
    def _apply_operation(self, operation: TransactionLog) -> None:
        """Apply a single operation"""
        # This is a simplified implementation
        # In a real system, this would modify the actual data
        pass
    
    def _load_transaction_history(self) -> None:
        """Load transaction history from storage"""
        try:
            import os
            from pathlib import Path
            
            history_file = Path(f"{self.storage_path}/transactions/transaction_history.json")
            if history_file.exists():
                with open(history_file, 'r') as f:
                    history_data = json.load(f)
                
                for tx_data in history_data.values():
                    if tx_data["state"] == "committed":
                        transaction = Transaction(
                            transaction_id=tx_data["transaction_id"],
                            state=TransactionState.COMMITTED,
                            isolation_level=IsolationLevel(tx_data["isolation_level"]),
                            created_at=datetime.fromisoformat(tx_data["created_at"]),
                            committed_at=datetime.fromisoformat(tx_data["committed_at"]) if tx_data["committed_at"] else None
                        )
                        self.committed_transactions[transaction.transaction_id] = transaction
        except Exception as e:
            print(f"Warning: Could not load transaction history: {e}")
    
    def _save_transaction_history(self) -> None:
        """Save transaction history to storage"""
        try:
            import os
            from pathlib import Path
            
            history_dir = Path(f"{self.storage_path}/transactions")
            history_dir.mkdir(parents=True, exist_ok=True)
            
            history_file = history_dir / "transaction_history.json"
            
            # Save both active and committed transactions
            all_transactions = {**self.active_transactions, **self.committed_transactions}
            
            history_data = {}
            for tx_id, transaction in all_transactions.items():
                history_data[tx_id] = transaction.to_dict()
            
            with open(history_file, 'w') as f:
                json.dump(history_data, f, indent=2, default=str)
        except Exception as e:
            print(f"Warning: Could not save transaction history: {e}")


@contextmanager
def transaction_context(transaction_manager: TransactionManager, 
                       isolation_level: IsolationLevel = IsolationLevel.READ_COMMITTED):
    """Context manager for transactions"""
    transaction_id = transaction_manager.begin_transaction(isolation_level)
    try:
        yield transaction_id
        transaction_manager.commit_transaction(transaction_id)
    except Exception as e:
        transaction_manager.rollback_transaction(transaction_id, str(e))
        raise
