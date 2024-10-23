import typer

app = typer.Typer(name="ezplugins", pretty_exceptions_show_locals=False, pretty_exceptions_short=True)


@app.command(name="mount")
def mount() -> None:
  """
  Mount your plugins type hints
  """

  from ezpz_pluginz import mount_plugins

  mount_plugins()


@app.command()
def unmount() -> None:
  """
  Unmount your plugins type hints
  """
  from ezpz_pluginz import unmount_plugins

  unmount_plugins()
