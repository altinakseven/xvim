" Test Vim script file

" Set some variables
let g:test_string = "Hello, world!"
let g:test_number = 42
let g:test_float = 3.14
let g:test_list = [1, 2, 3, 4, 5]
let g:test_dict = {'foo': 'bar', 'baz': 42}

" Define a function
function! TestFunction(arg1, arg2)
  echo "Argument 1: " . a:arg1
  echo "Argument 2: " . a:arg2
  return a:arg1 . a:arg2
endfunction

" Test if statement
if g:test_number > 0
  echo "test_number is positive"
else
  echo "test_number is not positive"
endif

" Test while loop
let g:counter = 0
while g:counter < 5
  let g:counter += 1
  echo "Counter: " . g:counter
endwhile

" Test for loop
let g:sum = 0
for i in g:test_list
  let g:sum += i
  echo "Sum: " . g:sum
endfor

" Set some options
set tabstop=4
set shiftwidth=4
set expandtab
set number
set relativenumber
set cursorline
set ignorecase
set smartcase
set incsearch
set hlsearch
set wrap
set linebreak
set showmatch
set showcmd
set showmode
set ruler
set wildmenu
set wildmode=list:longest
set laststatus=2
set statusline=%f\ %h%w%m%r\ %=%(%l,%c%V\ %=\ %P%)

" Print a message
echo "Test script executed successfully!"