all: libcstuff.so

libcstuff.so: libcstuff.c
	${CC} -shared ${CFLAGS} $< -o $@

clean:
	rm -rf libcstuff.so
