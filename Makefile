dep:
	yay -S openresty lua51

luarocks-install:
	luarocks --local --lua-version=5.1 install lapis luabitop uuid

run-api:
	(cd api/ && lapis server)

run-client:
	(cd client/ && lapis server)
