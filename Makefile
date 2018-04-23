version = 0.1.0

man/sac.1: MANUAL.md man/sac.1.template
	pandoc -f markdown-smart -t man -s \
		--template man/sac.1.template \
		--lua-filter man/manfilter.lua \
		--variable version="sac $(version)" \
		-o $@ \
		MANUAL.md
