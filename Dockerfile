FROM node:18.9.1-alpine

WORKDIR /usr/src/big-brother

COPY package*.json ./

RUN npm install

COPY . .

CMD [ "node", "." ]
