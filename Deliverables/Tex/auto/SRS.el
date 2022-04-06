(TeX-add-style-hook
 "SRS"
 (lambda ()
   (TeX-add-to-alist 'LaTeX-provided-class-options
                     '(("article" "11pt") ("document" "10pt" "a4paper" "showtrims")))
   (TeX-add-to-alist 'LaTeX-provided-package-options
                     '(("inputenc" "utf8") ("fontenc" "T1") ("ulem" "normalem") ("geometry" "margin=1in") ("caption" "labelfont=bf") ("biblatex" "natbib=true") ("forest" "linguistics")))
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "path")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "url")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "nolinkurl")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "hyperbaseurl")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "hyperimage")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "hyperref")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "href")
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
    "sec:org5a117c9"
    "sec:orgfd32af1"
    "sec:orgffff930"
    "sec:org4116edd"
    "sec:orgd80f1fe"
    "sec:org5ce12bf"
    "sec:org75faee9"
    "sec:orga5f9dc7"
    "sec:org2898fa9"
    "sec:org69bfba2"
    "sec:org4c2e530"
    "sec:org747ed52"
    "sec:org7ed9e72"
    "sec:org95774d1"
    "sec:orgf15a5c1"
    "sec:org0b87343"
    "sec:org7d8a8ab"
    "sec:org4a58e6b"
    "sec:org738f8b9"
    "sec:org769a772"
    "sec:orga4fd58b"
    "sec:org1723b22"
    "sec:orgf891884"
    "sec:orga2aede4"
    "sec:orgfd58507"
    "sec:orgb6d1ea7"
    "sec:org02114c2"
    "sec:org96b3ab0"
    "sec:org348ba7f"
    "sec:orga173c94"
    "sec:org33341c9"
    "sec:orgfef02e7"
    "sec:org6317d5d"
    "sec:org38de81e"
    "sec:orga8f48cd"))
 :latex)

