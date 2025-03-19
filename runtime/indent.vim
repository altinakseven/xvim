" Vim indentation file
" This file is loaded at startup to set up indentation settings

if exists("did_load_indent")
  finish
endif
let did_load_indent = 1

" Enable filetype detection, plugin, and indent
filetype plugin indent on

" Set default indentation settings
set autoindent      " Copy indent from current line when starting a new line
set smartindent     " Smart autoindenting when starting a new line
set cindent         " Stricter indentation rules for C programs
set expandtab       " Use spaces instead of tabs
set tabstop=4       " Number of spaces that a <Tab> counts for
set shiftwidth=4    " Number of spaces to use for each step of (auto)indent
set softtabstop=4   " Number of spaces that a <Tab> counts for while editing

" Set filetype-specific indentation settings
augroup indent
  autocmd!
  
  " C/C++
  autocmd FileType c,cpp setlocal cindent
  autocmd FileType c,cpp setlocal tabstop=4
  autocmd FileType c,cpp setlocal shiftwidth=4
  autocmd FileType c,cpp setlocal softtabstop=4
  
  " Java
  autocmd FileType java setlocal cindent
  autocmd FileType java setlocal tabstop=4
  autocmd FileType java setlocal shiftwidth=4
  autocmd FileType java setlocal softtabstop=4
  
  " Python
  autocmd FileType python setlocal autoindent
  autocmd FileType python setlocal tabstop=4
  autocmd FileType python setlocal shiftwidth=4
  autocmd FileType python setlocal softtabstop=4
  
  " Ruby
  autocmd FileType ruby setlocal autoindent
  autocmd FileType ruby setlocal tabstop=2
  autocmd FileType ruby setlocal shiftwidth=2
  autocmd FileType ruby setlocal softtabstop=2
  
  " JavaScript/TypeScript
  autocmd FileType javascript,typescript setlocal autoindent
  autocmd FileType javascript,typescript setlocal tabstop=2
  autocmd FileType javascript,typescript setlocal shiftwidth=2
  autocmd FileType javascript,typescript setlocal softtabstop=2
  
  " HTML/XML
  autocmd FileType html,xml,xhtml,svg setlocal autoindent
  autocmd FileType html,xml,xhtml,svg setlocal tabstop=2
  autocmd FileType html,xml,xhtml,svg setlocal shiftwidth=2
  autocmd FileType html,xml,xhtml,svg setlocal softtabstop=2
  
  " CSS/SCSS/LESS
  autocmd FileType css,scss,less setlocal autoindent
  autocmd FileType css,scss,less setlocal tabstop=2
  autocmd FileType css,scss,less setlocal shiftwidth=2
  autocmd FileType css,scss,less setlocal softtabstop=2
  
  " YAML
  autocmd FileType yaml setlocal autoindent
  autocmd FileType yaml setlocal tabstop=2
  autocmd FileType yaml setlocal shiftwidth=2
  autocmd FileType yaml setlocal softtabstop=2
  
  " JSON
  autocmd FileType json setlocal autoindent
  autocmd FileType json setlocal tabstop=2
  autocmd FileType json setlocal shiftwidth=2
  autocmd FileType json setlocal softtabstop=2
  
  " Markdown
  autocmd FileType markdown setlocal autoindent
  autocmd FileType markdown setlocal tabstop=4
  autocmd FileType markdown setlocal shiftwidth=4
  autocmd FileType markdown setlocal softtabstop=4
  
  " Shell scripts
  autocmd FileType sh,bash,zsh setlocal autoindent
  autocmd FileType sh,bash,zsh setlocal tabstop=2
  autocmd FileType sh,bash,zsh setlocal shiftwidth=2
  autocmd FileType sh,bash,zsh setlocal softtabstop=2
  
  " Rust
  autocmd FileType rust setlocal cindent
  autocmd FileType rust setlocal tabstop=4
  autocmd FileType rust setlocal shiftwidth=4
  autocmd FileType rust setlocal softtabstop=4
  
  " Go
  autocmd FileType go setlocal noexpandtab
  autocmd FileType go setlocal tabstop=4
  autocmd FileType go setlocal shiftwidth=4
  
  " Lua
  autocmd FileType lua setlocal autoindent
  autocmd FileType lua setlocal tabstop=2
  autocmd FileType lua setlocal shiftwidth=2
  autocmd FileType lua setlocal softtabstop=2
  
  " Vim script
  autocmd FileType vim setlocal autoindent
  autocmd FileType vim setlocal tabstop=2
  autocmd FileType vim setlocal shiftwidth=2
  autocmd FileType vim setlocal softtabstop=2
  
  " Makefile (requires tabs)
  autocmd FileType make setlocal noexpandtab
  autocmd FileType make setlocal tabstop=8
  autocmd FileType make setlocal shiftwidth=8
  
  " CMake
  autocmd FileType cmake setlocal autoindent
  autocmd FileType cmake setlocal tabstop=2
  autocmd FileType cmake setlocal shiftwidth=2
  autocmd FileType cmake setlocal softtabstop=2
  
  " Dockerfile
  autocmd FileType dockerfile setlocal autoindent
  autocmd FileType dockerfile setlocal tabstop=4
  autocmd FileType dockerfile setlocal shiftwidth=4
  autocmd FileType dockerfile setlocal softtabstop=4
  
  " TOML
  autocmd FileType toml setlocal autoindent
  autocmd FileType toml setlocal tabstop=2
  autocmd FileType toml setlocal shiftwidth=2
  autocmd FileType toml setlocal softtabstop=2
  
  " INI
  autocmd FileType ini setlocal autoindent
  autocmd FileType ini setlocal tabstop=4
  autocmd FileType ini setlocal shiftwidth=4
  autocmd FileType ini setlocal softtabstop=4
  
  " Configuration files
  autocmd FileType conf,config setlocal autoindent
  autocmd FileType conf,config setlocal tabstop=4
  autocmd FileType conf,config setlocal shiftwidth=4
  autocmd FileType conf,config setlocal softtabstop=4
  
  " Git config
  autocmd FileType gitconfig setlocal noexpandtab
  autocmd FileType gitconfig setlocal tabstop=8
  autocmd FileType gitconfig setlocal shiftwidth=8
  
  " SQL
  autocmd FileType sql setlocal autoindent
  autocmd FileType sql setlocal tabstop=4
  autocmd FileType sql setlocal shiftwidth=4
  autocmd FileType sql setlocal softtabstop=4
  
  " LaTeX
  autocmd FileType tex setlocal autoindent
  autocmd FileType tex setlocal tabstop=2
  autocmd FileType tex setlocal shiftwidth=2
  autocmd FileType tex setlocal softtabstop=2
  
  " BibTeX
  autocmd FileType bib setlocal autoindent
  autocmd FileType bib setlocal tabstop=2
  autocmd FileType bib setlocal shiftwidth=2
  autocmd FileType bib setlocal softtabstop=2
  
  " Haskell
  autocmd FileType haskell setlocal autoindent
  autocmd FileType haskell setlocal tabstop=2
  autocmd FileType haskell setlocal shiftwidth=2
  autocmd FileType haskell setlocal softtabstop=2
  
  " Erlang
  autocmd FileType erlang setlocal autoindent
  autocmd FileType erlang setlocal tabstop=4
  autocmd FileType erlang setlocal shiftwidth=4
  autocmd FileType erlang setlocal softtabstop=4
  
  " Elixir
  autocmd FileType elixir setlocal autoindent
  autocmd FileType elixir setlocal tabstop=2
  autocmd FileType elixir setlocal shiftwidth=2
  autocmd FileType elixir setlocal softtabstop=2
  
  " Clojure
  autocmd FileType clojure setlocal autoindent
  autocmd FileType clojure setlocal tabstop=2
  autocmd FileType clojure setlocal shiftwidth=2
  autocmd FileType clojure setlocal softtabstop=2
  
  " Scala
  autocmd FileType scala setlocal autoindent
  autocmd FileType scala setlocal tabstop=2
  autocmd FileType scala setlocal shiftwidth=2
  autocmd FileType scala setlocal softtabstop=2
  
  " Swift
  autocmd FileType swift setlocal autoindent
  autocmd FileType swift setlocal tabstop=4
  autocmd FileType swift setlocal shiftwidth=4
  autocmd FileType swift setlocal softtabstop=4
  
  " Objective-C
  autocmd FileType objc setlocal cindent
  autocmd FileType objc setlocal tabstop=4
  autocmd FileType objc setlocal shiftwidth=4
  autocmd FileType objc setlocal softtabstop=4
  
  " R
  autocmd FileType r setlocal autoindent
  autocmd FileType r setlocal tabstop=2
  autocmd FileType r setlocal shiftwidth=2
  autocmd FileType r setlocal softtabstop=2
  
  " Julia
  autocmd FileType julia setlocal autoindent
  autocmd FileType julia setlocal tabstop=4
  autocmd FileType julia setlocal shiftwidth=4
  autocmd FileType julia setlocal softtabstop=4
  
  " Dart
  autocmd FileType dart setlocal autoindent
  autocmd FileType dart setlocal tabstop=2
  autocmd FileType dart setlocal shiftwidth=2
  autocmd FileType dart setlocal softtabstop=2
  
  " Kotlin
  autocmd FileType kotlin setlocal autoindent
  autocmd FileType kotlin setlocal tabstop=4
  autocmd FileType kotlin setlocal shiftwidth=4
  autocmd FileType kotlin setlocal softtabstop=4
  
  " Groovy
  autocmd FileType groovy setlocal autoindent
  autocmd FileType groovy setlocal tabstop=4
  autocmd FileType groovy setlocal shiftwidth=4
  autocmd FileType groovy setlocal softtabstop=4
  
  " Terraform
  autocmd FileType terraform setlocal autoindent
  autocmd FileType terraform setlocal tabstop=2
  autocmd FileType terraform setlocal shiftwidth=2
  autocmd FileType terraform setlocal softtabstop=2
  
  " HCL
  autocmd FileType hcl setlocal autoindent
  autocmd FileType hcl setlocal tabstop=2
  autocmd FileType hcl setlocal shiftwidth=2
  autocmd FileType hcl setlocal softtabstop=2
  
  " Nginx
  autocmd FileType nginx setlocal autoindent
  autocmd FileType nginx setlocal tabstop=4
  autocmd FileType nginx setlocal shiftwidth=4
  autocmd FileType nginx setlocal softtabstop=4
  
  " Apache
  autocmd FileType apache setlocal autoindent
  autocmd FileType apache setlocal tabstop=4
  autocmd FileType apache setlocal shiftwidth=4
  autocmd FileType apache setlocal softtabstop=4
  
  " Protocol Buffers
  autocmd FileType proto setlocal autoindent
  autocmd FileType proto setlocal tabstop=2
  autocmd FileType proto setlocal shiftwidth=2
  autocmd FileType proto setlocal softtabstop=2
  
  " GraphQL
  autocmd FileType graphql setlocal autoindent
  autocmd FileType graphql setlocal tabstop=2
  autocmd FileType graphql setlocal shiftwidth=2
  autocmd FileType graphql setlocal softtabstop=2
  
  " Svelte
  autocmd FileType svelte setlocal autoindent
  autocmd FileType svelte setlocal tabstop=2
  autocmd FileType svelte setlocal shiftwidth=2
  autocmd FileType svelte setlocal softtabstop=2
  
  " Vue
  autocmd FileType vue setlocal autoindent
  autocmd FileType vue setlocal tabstop=2
  autocmd FileType vue setlocal shiftwidth=2
  autocmd FileType vue setlocal softtabstop=2
  
  " Templating languages
  autocmd FileType pug,jade,ejs,handlebars,mustache,twig,liquid,nunjucks,smarty,blade,haml,slim setlocal autoindent
  autocmd FileType pug,jade,ejs,handlebars,mustache,twig,liquid,nunjucks,smarty,blade,haml,slim setlocal tabstop=2
  autocmd FileType pug,jade,ejs,handlebars,mustache,twig,liquid,nunjucks,smarty,blade,haml,slim setlocal shiftwidth=2
  autocmd FileType pug,jade,ejs,handlebars,mustache,twig,liquid,nunjucks,smarty,blade,haml,slim setlocal softtabstop=2
  
  " Functional languages
  autocmd FileType ocaml,fsharp,elm,purescript,reason setlocal autoindent
  autocmd FileType ocaml,fsharp,elm,purescript,reason setlocal tabstop=2
  autocmd FileType ocaml,fsharp,elm,purescript,reason setlocal shiftwidth=2
  autocmd FileType ocaml,fsharp,elm,purescript,reason setlocal softtabstop=2
  
  " Newer languages
  autocmd FileType crystal,nim,d,vala,zig,v,vlang,odin,janet,io,idris,agda,coq,isabelle,lean,dhall,nix setlocal autoindent
  autocmd FileType crystal,nim,d,vala,zig,v,vlang,odin,janet,io,idris,agda,coq,isabelle,lean,dhall,nix setlocal tabstop=2
  autocmd FileType crystal,nim,d,vala,zig,v,vlang,odin,janet,io,idris,agda,coq,isabelle,lean,dhall,nix setlocal shiftwidth=2
  autocmd FileType crystal,nim,d,vala,zig,v,vlang,odin,janet,io,idris,agda,coq,isabelle,lean,dhall,nix setlocal softtabstop=2
  
  " Legacy languages
  autocmd FileType ada,cobol,fortran,pascal,prolog setlocal autoindent
  autocmd FileType ada,cobol,fortran,pascal,prolog setlocal tabstop=4
  autocmd FileType ada,cobol,fortran,pascal,prolog setlocal shiftwidth=4
  autocmd FileType ada,cobol,fortran,pascal,prolog setlocal softtabstop=4
  
  " Lisp-like languages
  autocmd FileType lisp,scheme,racket,fennel setlocal autoindent
  autocmd FileType lisp,scheme,racket,fennel setlocal tabstop=2
  autocmd FileType lisp,scheme,racket,fennel setlocal shiftwidth=2
  autocmd FileType lisp,scheme,racket,fennel setlocal softtabstop=2
  
  " System languages
  autocmd FileType tcl,awk,sed,m4,yacc,lex,bison,flex,antlr,abnf,ebnf,bnf,ragel setlocal autoindent
  autocmd FileType tcl,awk,sed,m4,yacc,lex,bison,flex,antlr,abnf,ebnf,bnf,ragel setlocal tabstop=4
  autocmd FileType tcl,awk,sed,m4,yacc,lex,bison,flex,antlr,abnf,ebnf,bnf,ragel setlocal shiftwidth=4
  autocmd FileType tcl,awk,sed,m4,yacc,lex,bison,flex,antlr,abnf,ebnf,bnf,ragel setlocal softtabstop=4
  
  " Financial languages
  autocmd FileType beancount,ledger setlocal autoindent
  autocmd FileType beancount,ledger setlocal tabstop=2
  autocmd FileType beancount,ledger setlocal shiftwidth=2
  autocmd FileType beancount,ledger setlocal softtabstop=2
  
  " Hardware description languages
  autocmd FileType verilog,vhdl,systemverilog,spice setlocal autoindent
  autocmd FileType verilog,vhdl,systemverilog,spice setlocal tabstop=2
  autocmd FileType verilog,vhdl,systemverilog,spice setlocal shiftwidth=2
  autocmd FileType verilog,vhdl,systemverilog,spice setlocal softtabstop=2
  
  " Scientific computing languages
  autocmd FileType matlab,octave,scilab,gnuplot,mma,maple,sage,maxima,modelica,stan,stata,sas,spss setlocal autoindent
  autocmd FileType matlab,octave,scilab,gnuplot,mma,maple,sage,maxima,modelica,stan,stata,sas,spss setlocal tabstop=2
  autocmd FileType matlab,octave,scilab,gnuplot,mma,maple,sage,maxima,modelica,stan,stata,sas,spss setlocal shiftwidth=2
  autocmd FileType matlab,octave,scilab,gnuplot,mma,maple,sage,maxima,modelica,stan,stata,sas,spss setlocal softtabstop=2
  
  " Help files
  autocmd FileType help setlocal noexpandtab
  autocmd FileType help setlocal tabstop=8
  autocmd FileType help setlocal shiftwidth=8
augroup END
