FROM "ubuntu"

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    gcc \
    libssl-dev build-essential checkinstall zlib1g-dev # openssl deps

# install rust
RUN curl https://sh.rustup.rs -sSf > rustup.sh
RUN chmod 755 rustup.sh
RUN ./rustup.sh -y

# build rust-wasm package
COPY . /app
RUN ~/.cargo/bin/cargo install wasm-pack
RUN ~/.cargo/bin/wasm-pack build

# install node
ARG NODE_VERSION=14.16.0
ARG NODE_PACKAGE=node-v$NODE_VERSION-linux-x64
ARG NODE_HOME=/opt/$NODE_PACKAGE

ENV NODE_PATH $NODE_HOME/lib/node_modules
ENV PATH $NODE_HOME/bin:$PATH

RUN curl https://nodejs.org/dist/v$NODE_VERSION/$NODE_PACKAGE.tar.gz | tar -xzC /opt/

# build npm project
RUN npm i -g typescript
COPY ./www /app/www
RUN npm i

WORKDIR /app/www
RUN tsc

CMD [ "npm", "run", "start" ]
