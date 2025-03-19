" Name: Monokai
" Description: Monokai color scheme for xvim
" Author: xvim Team

" Reset all highlighting to the defaults
highlight clear

" Set the background
set background=dark

" Set the color scheme name
let g:colors_name = "monokai"

" Normal text
highlight Normal guifg=#F8F8F2 guibg=#272822

" Syntax highlighting groups
highlight Comment guifg=#75715E gui=italic
highlight Constant guifg=#AE81FF
highlight String guifg=#E6DB74
highlight Character guifg=#E6DB74
highlight Number guifg=#AE81FF
highlight Boolean guifg=#AE81FF
highlight Float guifg=#AE81FF
highlight Identifier guifg=#66D9EF
highlight Function guifg=#A6E22E
highlight Statement guifg=#F92672
highlight Conditional guifg=#F92672
highlight Repeat guifg=#F92672
highlight Label guifg=#F92672
highlight Operator guifg=#F92672
highlight Keyword guifg=#F92672
highlight Exception guifg=#F92672
highlight PreProc guifg=#A6E22E
highlight Include guifg=#A6E22E
highlight Define guifg=#A6E22E
highlight Macro guifg=#A6E22E
highlight PreCondit guifg=#A6E22E
highlight Type guifg=#66D9EF
highlight StorageClass guifg=#66D9EF
highlight Structure guifg=#66D9EF
highlight Typedef guifg=#66D9EF
highlight Special guifg=#FD971F
highlight SpecialChar guifg=#FD971F
highlight Tag guifg=#F92672
highlight Delimiter guifg=#F8F8F2
highlight SpecialComment guifg=#75715E gui=italic
highlight Debug guifg=#FD971F
highlight Underlined guifg=#66D9EF gui=underline
highlight Error guifg=#F8F8F2 guibg=#F92672
highlight Todo guifg=#F8F8F2 guibg=#75715E gui=bold

" UI highlighting
highlight ColorColumn guibg=#3E3D32
highlight Cursor guifg=#272822 guibg=#F8F8F2
highlight CursorLine guibg=#3E3D32
highlight CursorLineNr guifg=#FD971F guibg=#3E3D32
highlight Directory guifg=#66D9EF
highlight DiffAdd guifg=#F8F8F2 guibg=#468410
highlight DiffChange guifg=#F8F8F2 guibg=#3E3D32
highlight DiffDelete guifg=#F92672 guibg=#3E3D32
highlight DiffText guifg=#F8F8F2 guibg=#FD971F
highlight ErrorMsg guifg=#F8F8F2 guibg=#F92672
highlight VertSplit guifg=#75715E guibg=NONE
highlight Folded guifg=#75715E guibg=#3E3D32
highlight FoldColumn guifg=#75715E guibg=#272822
highlight SignColumn guifg=NONE guibg=#272822
highlight IncSearch guifg=#272822 guibg=#E6DB74
highlight LineNr guifg=#75715E guibg=#272822
highlight MatchParen guifg=#F8F8F2 guibg=#FD971F
highlight ModeMsg guifg=#E6DB74
highlight MoreMsg guifg=#E6DB74
highlight NonText guifg=#75715E
highlight Pmenu guifg=#F8F8F2 guibg=#3E3D32
highlight PmenuSel guifg=#272822 guibg=#FD971F
highlight PmenuSbar guifg=NONE guibg=#3E3D32
highlight PmenuThumb guifg=NONE guibg=#75715E
highlight Question guifg=#A6E22E
highlight Search guifg=#272822 guibg=#E6DB74
highlight SpecialKey guifg=#75715E
highlight SpellBad guifg=#F92672 gui=underline
highlight SpellCap guifg=#E6DB74 gui=underline
highlight SpellRare guifg=#AE81FF gui=underline
highlight SpellLocal guifg=#66D9EF gui=underline
highlight StatusLine guifg=#F8F8F2 guibg=#3E3D32
highlight StatusLineNC guifg=#75715E guibg=#3E3D32
highlight TabLine guifg=#75715E guibg=#3E3D32
highlight TabLineFill guifg=NONE guibg=#3E3D32
highlight TabLineSel guifg=#F8F8F2 guibg=#272822
highlight Title guifg=#A6E22E
highlight Visual guifg=NONE guibg=#49483E
highlight VisualNOS guifg=NONE guibg=#49483E
highlight WarningMsg guifg=#FD971F
highlight WildMenu guifg=#272822 guibg=#FD971F