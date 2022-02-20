(TeX-add-style-hook
 "SRS"
 (lambda ()
   (TeX-add-to-alist 'LaTeX-provided-class-options
                     '(("article" "11pt") ("document" "10pt" "a4paper" "showtrims")))
   (TeX-add-to-alist 'LaTeX-provided-package-options
                     '(("inputenc" "utf8") ("fontenc" "T1") ("ulem" "normalem") ("geometry" "margin=1in") ("caption" "labelfont=bf") ("biblatex" "natbib=true") ("forest" "linguistics")))
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "href")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "hyperref")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "hyperimage")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "hyperbaseurl")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "nolinkurl")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "url")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "path")
   (add-to-list 'LaTeX-verbatim-macros-with-delims-local "path")
   (TeX-run-style-hooks
    "latex2e"
    "article"
    "art11"
    "inputenc"
    "fontenc"
    "graphicx"
    "grffile"
    "longtable"
    "wrapfig"
    "rotating"
    "ulem"
    "amsmath"
    "textcomp"
    "amssymb"
    "capt-of"
    "hyperref"
    "geometry"
    "enumitem"
    "document"
    "document10"
    "caption"
    "biblatex"
    "tikz"
    "forest"
    "sectsty"
    "parskip")
   (LaTeX-add-labels
    "sec:orgaef73a0"
    "sec:org7a998c5"
    "sec:orgfb60f87"
    "sec:orga8df548"
    "sec:org1fa0c88"
    "sec:org33df046"
    "sec:org641512a"
    "sec:org3ee4ccc"
    "sec:orga0329fd"
    "sec:org8dfae52"
    "sec:org63cdcb8"
    "sec:org5a38786"
    "sec:orgdf3205b"
    "sec:orgc500afa"
    "sec:org942da58"
    "sec:org8b629b2"
    "sec:orgfdea334"
    "sec:org639c6bb"
    "sec:org0cabcae"
    "sec:orgf33e8f5"
    "sec:orgce40aa4"
    "sec:org49d646e"
    "sec:org46dbc81"
    "sec:org7e9a25a"
    "sec:org3b55e13"
    "sec:org8a105f4"
    "sec:org608e25b"
    "sec:org9d223a8"
    "sec:org4336555"
    "sec:orgd1fbeb5"
    "sec:orgaa62b25"
    "sec:org5cf1f28"
    "sec:org9eda1b8"
    "sec:orgab4db96"
    "sec:org9e93733"))
 :latex)

