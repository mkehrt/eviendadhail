out/lexicon.pdf: out/lexicon.tex out
	pdflatex -output-directory out out/lexicon.tex

out/lexicon.tex: src/main.rs data/prelude.tex data/words.yaml data/postlude.tex out
	cargo run -- \
  	--prelude data/prelude.tex \
  	--words data/words.yaml \
  	--postlude data/postlude.tex \
  	--output out/lexicon.tex

out:
	mkdir out

clean:
	rm -rf out
	cargo clean
