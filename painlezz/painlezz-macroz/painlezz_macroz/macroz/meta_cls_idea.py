from abc import ABCMeta
from typing import Type, TypeVar

T = TypeVar("T", bound="RegisteredClass")


class RegisteredMeta(ABCMeta):
  def __new__(mcs, name: str, bases: tuple[type, ...], namespace: dict[str, object]) -> Type[T]:
    cls = super().__new__(mcs, name, bases, namespace)
    RegisteredClass.register(cls)
    print(f"Class {name} has been created and registered")
    return cls


class RegisteredClass(metaclass=RegisteredMeta):
  pass


# Example usage:
class ExampleClass(RegisteredClass):
  def __init__(self, value: int) -> None:
    self.value = value


class AnotherClass(RegisteredClass):
  def __init__(self, name: str) -> None:
    self.name = name


# Virtual subclass (not a direct subclass)
class VirtualClass:
  pass


RegisteredClass.register(VirtualClass)

# Print subclasses
print("Direct subclasses:", RegisteredClass.__subclasses__())
print("All subclasses:", RegisteredClass.__subclasses__() + list(RegisteredClass._abc_registry))
