dep:
	yay -S openresty lua51

luarocks-install:
	luarocks --local --lua-version=5.1 install lapis luabitop uuid

start-api:
	(cd api/ && lapis server)

stop-api:
	(cd api/ && lapis term)
