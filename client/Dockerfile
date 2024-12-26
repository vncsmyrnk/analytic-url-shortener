FROM openresty/openresty:alpine
RUN apk add --no-cache lua5.1 luarocks
RUN <<EOF
luarocks install lapis luabitop
EOF
COPY . /app
WORKDIR /app
EXPOSE 8080
CMD ["lapis", "server", "production"]
