---
title: Replace all imports of AsyncSession from sqlalchemy to sqlmodel
level: error
---

Prevent import from sqlalchemy when need to import from sqlmodel.

```grit
engine marzano(0.1)
language python

or {
  py_find_replace_import(
    from_package=`sqlalchemy.ext.asyncio`,
    from_name=`AsyncSession`,
    to_package=`sqlmodel.ext.asyncio.session`,
    to_name=`AsyncSession`
  )
}
```

## Test case one

```python
from sqlalchemy.ext.asyncio import AsyncSession
```

```python
from sqlmodel.ext.asyncio.session import AsyncSession
```

## Test case two

```python
from sqlalchemy.ext.asyncio import AsyncSession, async_sessionmaker, create_async_engine
```

```python
from sqlalchemy.ext.asyncio import  async_sessionmaker, create_async_engine

from sqlmodel.ext.asyncio.session import AsyncSession
```
