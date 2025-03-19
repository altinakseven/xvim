" Name: Solarized
" Description: Solarized color scheme for xvim
" Author: xvim Team

" Reset all highlighting to the defaults
highlight clear

" Set the background
set background=dark

" Set the color scheme name
let g:colors_name = "solarized"

" Solarized color palette
" base03    #002b36
" base02    #073642
" base01    #586e75
" base00    #657b83
" base0     #839496
" base1     #93a1a1
" base2     #eee8d5
" base3     #fdf6e3
" yellow    #b58900
" orange    #cb4b16
" red       #dc322f
" magenta   #d33682
" violet    #6c71c4
" blue      #268bd2
" cyan      #2aa198
" green     #859900

" Normal text
highlight Normal guifg=#839496 guibg=#002b36

" Syntax highlighting groups
highlight Comment guifg=#586e75 gui=italic
highlight Constant guifg=#2aa198
highlight String guifg=#2aa198
highlight Character guifg=#2aa198
highlight Number guifg=#d33682
highlight Boolean guifg=#d33682
highlight Float guifg=#d33682
highlight Identifier guifg=#268bd2
highlight Function guifg=#268bd2
highlight Statement guifg=#859900
highlight Conditional guifg=#859900
highlight Repeat guifg=#859900
highlight Label guifg=#859900
highlight Operator guifg=#859900
highlight Keyword guifg=#859900
highlight Exception guifg=#859900
highlight PreProc guifg=#cb4b16
highlight Include guifg=#cb4b16
highlight Define guifg=#cb4b16
highlight Macro guifg=#cb4b16
highlight PreCondit guifg=#cb4b16
highlight Type guifg=#b58900
highlight StorageClass guifg=#b58900
highlight Structure guifg=#b58900
highlight Typedef guifg=#b58900
highlight Special guifg=#dc322f
highlight SpecialChar guifg=#dc322f
highlight Tag guifg=#268bd2
highlight Delimiter guifg=#839496
highlight SpecialComment guifg=#586e75 gui=italic
highlight Debug guifg=#dc322f
highlight Underlined guifg=#268bd2 gui=underline
highlight Error guifg=#dc322f guibg=NONE
highlight Todo guifg=#b58900 guibg=NONE gui=bold

" UI highlighting
highlight ColorColumn guibg=#073642
highlight Cursor guifg=#002b36 guibg=#839496
highlight CursorLine guibg=#073642
highlight CursorLineNr guifg=#93a1a1 guibg=#073642
highlight Directory guifg=#268bd2
highlight DiffAdd guifg=#859900 guibg=#073642
highlight DiffChange guifg=#b58900 guibg=#073642
highlight DiffDelete guifg=#dc322f guibg=#073642
highlight DiffText guifg=#268bd2 guibg=#073642
highlight ErrorMsg guifg=#dc322f guibg=NONE
highlight VertSplit guifg=#586e75 guibg=NONE
highlight Folded guifg=#586e75 guibg=#073642
highlight FoldColumn guifg=#586e75 guibg=#002b36
highlight SignColumn guifg=NONE guibg=#002b36
highlight IncSearch guifg=#002b36 guibg=#b58900
highlight LineNr guifg=#586e75 guibg=#002b36
highlight MatchParen guifg=#002b36 guibg=#586e75
highlight ModeMsg guifg=#268bd2
highlight MoreMsg guifg=#268bd2
highlight NonText guifg=#586e75
highlight Pmenu guifg=#839496 guibg=#073642
highlight PmenuSel guifg=#eee8d5 guibg=#586e75
highlight PmenuSbar guifg=NONE guibg=#073642
highlight PmenuThumb guifg=NONE guibg=#586e75
highlight Question guifg=#2aa198
highlight Search guifg=#002b36 guibg=#b58900
highlight SpecialKey guifg=#586e75
highlight SpellBad guifg=#dc322f gui=underline
highlight SpellCap guifg=#b58900 gui=underline
highlight SpellRare guifg=#d33682 gui=underline
highlight SpellLocal guifg=#2aa198 gui=underline
highlight StatusLine guifg=#eee8d5 guibg=#073642
highlight StatusLineNC guifg=#586e75 guibg=#073642
highlight TabLine guifg=#586e75 guibg=#073642
highlight TabLineFill guifg=NONE guibg=#073642
highlight TabLineSel guifg=#eee8d5 guibg=#073642
highlight Title guifg=#cb4b16
highlight Visual guifg=NONE guibg=#073642
highlight VisualNOS guifg=NONE guibg=#073642
highlight WarningMsg guifg=#cb4b16
highlight WildMenu guifg=#eee8d5 guibg=#073642