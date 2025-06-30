from alembic_utils.pg_function import PGFunction

uuid_generate_v4_function = PGFunction(
  schema="public",
  signature="uuid_generate_v4()",
  definition="""
    RETURNS uuid AS
    $$
    SELECT uuid_generate_v4()
    $$ LANGUAGE sql VOLATILE;
    """,
)
