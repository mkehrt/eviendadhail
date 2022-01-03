convert: out/lexicon.pdf

format: src/lib/* src/format/* in/words.yaml
	cargo run --bin format -- \
	--words in/words.yaml

out/lexicon.pdf: out/lexicon.tex out
	pdflatex -output-directory out out/lexicon.tex

out/lexicon.tex: src/lib/* src/convert/* in/* out
	cargo run --bin convert -- \
  	--prelude in/prelude.tex \
  	--words in/words.yaml \
  	--postlude in/postlude.tex \
  	--output in/lexicon.tex

out:
	mkdir out

clean:
	rm -rf out
	cargo clean
