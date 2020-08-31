FROM secondstate/ssvm-nodejs-starter:v1 as builder

COPY . /app

WORKDIR /app
RUN ssvmup build

FROM secondstate/ssvm-nodejs-starter:v1

VOLUME [ "/app" ]

COPY --from=builder /app/pkg /app/pkg
COPY --from=builder /app/node/app.js /app/node/app.js

WORKDIR /app
EXPOSE 3000
CMD [ "node", "node/app.js" ]
