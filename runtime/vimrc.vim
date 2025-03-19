" Default vimrc.vim for xvim
" This file is loaded at startup

" Set default options
set nocompatible
set backspace=indent,eol,start
set history=50
set ruler
set showcmd
set incsearch
set hlsearch
set ignorecase
set smartcase
set tabstop=4
set shiftwidth=4
set expandtab
set number
set autoindent
set smartindent
set wrap
set linebreak
set showmatch
set wildmenu
set wildmode=list:longest
set laststatus=2
set statusline=%f\ %h%w%m%r\ %=%(%l,%c%V\ %=\ %P%)
set mouse=a
set clipboard=unnamed
set hidden
set backup
set writebackup
set swapfile
set undofile
set undolevels=1000
set undoreload=10000
set encoding=utf-8
set fileencoding=utf-8
set fileencodings=utf-8
set termencoding=utf-8
set fileformats=unix,dos,mac
set magic
set showmatch
set mat=2
set noerrorbells
set novisualbell
set t_vb=
set tm=500
set foldmethod=indent
set foldlevel=99
set foldenable
set foldlevelstart=10
set foldnestmax=10
set background=dark
set cursorline
set colorcolumn=80

" Define leader key
let mapleader = ","
let g:mapleader = ","

" Key mappings
nnoremap <leader>w :w<CR>
nnoremap <leader>q :q<CR>
nnoremap <leader>x :x<CR>
nnoremap <leader>e :e<Space>
nnoremap <leader>v :vsplit<CR>
nnoremap <leader>s :split<CR>
nnoremap <leader>h <C-w>h
nnoremap <leader>j <C-w>j
nnoremap <leader>k <C-w>k
nnoremap <leader>l <C-w>l

" Better navigation
nnoremap j gj
nnoremap k gk
nnoremap $ g$
nnoremap 0 g0
nnoremap ^ g^

" Quick save
inoremap <C-s> <Esc>:w<CR>
nnoremap <C-s> :w<CR>

" Quick quit
inoremap <C-q> <Esc>:q<CR>
nnoremap <C-q> :q<CR>

" Quick escape
inoremap jk <Esc>
inoremap kj <Esc>

" Auto commands
if has("autocmd")
    " Enable file type detection
    filetype plugin indent on

    " Automatically remove trailing whitespace on save
    autocmd BufWritePre * :%s/\s\+$//e

    " Automatically set the cursor to the last position when reopening a file
    autocmd BufReadPost *
        \ if line("'\"") > 0 && line("'\"") <= line("$") |
        \   exe "normal! g`\"" |
        \ endif
endif
