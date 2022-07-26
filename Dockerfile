FROM "rust"

WORKDIR /app
ADD . /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    gcc \
    libssl-dev build-essential checkinstall zlib1g-dev # openssl deps

# build rust-wasm package
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 
RUN wasm-pack build

# install node
ARG NODE_VERSION=14.16.0
ARG NODE_PACKAGE=node-v$NODE_VERSION-linux-x64
ARG NODE_HOME=/opt/$NODE_PACKAGE

ENV NODE_PATH $NODE_HOME/lib/node_modules
ENV PATH $NODE_HOME/bin:$PATH

RUN curl https://nodejs.org/dist/v$NODE_VERSION/$NODE_PACKAGE.tar.gz | tar -xzC /opt/

# build npm project
WORKDIR /app/www
RUN npm i -g typescript webpack webpack-dev-server webpack-cli
RUN npm i
RUN tsc

CMD [ "npm", "run", "start" ]
