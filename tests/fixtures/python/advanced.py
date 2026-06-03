"""Advanced Python module for testing ast-cli."""

from __future__ import annotations

import asyncio
from dataclasses import dataclass, field
from abc import ABC, abstractmethod
from typing import Any, Optional, TypeVar, Generic

T = TypeVar("T")

# ── Type Alias (Python 3.12+) ────────────────────────────
type Point = tuple[float, float]


# ── Dataclass decorator ─────────────────────────────────
@dataclass
class Config:
    host: str = "localhost"
    port: int = 8080
    debug: bool = False
    tags: list[str] = field(default_factory=list)


# ── Abstract base class ─────────────────────────────────
class BaseRepository(ABC, Generic[T]):
    """Abstract repository providing CRUD operations."""

    def __init__(self) -> None:
        self._store: dict[str, T] = {}

    @abstractmethod
    def validate(self, item: T) -> bool:
        ...

    def get(self, key: str) -> Optional[T]:
        return self._store.get(key)

    def put(self, key: str, item: T) -> None:
        if not self.validate(item):
            raise ValueError("Validation failed")
        self._store[key] = item

    @property
    def count(self) -> int:
        return len(self._store)


# ── Multiple inheritance ─────────────────────────────────
class Auditable:
    """Mixin for audit logging."""

    def log_action(self, action: str) -> None:
        print(f"AUDIT: {action}")


# ── Complex class: multiple inheritance + decorators ─────
class UserRepository(BaseRepository[dict[str, Any]], Auditable):
    """Concrete repository for user records."""

    def validate(self, item: dict[str, Any]) -> bool:
        return "name" in item and "email" in item

    def put(self, key: str, item: dict[str, Any]) -> None:
        super().put(key, item)
        self.log_action(f"created user {key}")

    @classmethod
    def create_default(cls) -> "UserRepository":
        return cls()

    @staticmethod
    def normalize_email(email: str) -> str:
        return email.strip().lower()

    # ── Inner class ──────────────────────────────────────
    class Permissions:
        READ = "read"
        WRITE = "write"
        ADMIN = "admin"

        def __init__(self, *roles: str) -> None:
            self.roles = set(roles)

        def has(self, role: str) -> bool:
            return role in self.roles


# ── Async function with complex signature ────────────────
async def fetch_users(
    base_url: str,
    *,
    limit: int = 100,
    offset: int = 0,
    filters: Optional[dict[str, Any]] = None,
) -> list[dict[str, Any]]:
    """Fetch users from a remote API."""
    await asyncio.sleep(0)
    return []


# ── Nested functions (closures) ──────────────────────────
def create_pipeline(*steps: Any) -> Any:
    """Create a processing pipeline."""

    def execute(data: Any) -> Any:
        result = data
        for step in steps:
            result = step(result)
        return result

    def describe() -> str:
        return " -> ".join(s.__name__ for s in steps)

    execute.describe = describe  # type: ignore
    return execute


# ── Multiple stacked decorators ──────────────────────────
@dataclass
class CacheEntry(Generic[T]):
    key: str
    value: T
    ttl: int = 300
