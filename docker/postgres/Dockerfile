FROM pgvector/pgvector:pg15
RUN apt-get update &&  apt-get install -y openssh-client &&  rm -rf /var/lib/apt/lists/*
COPY init.sql /docker-entrypoint-initdb.d/