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
    "sec:org9b34e07"
    "sec:orga38c260"
    "sec:orgc2e2695"
    "sec:org9845e43"
    "sec:org15def12"
    "sec:orgd54810b"
    "sec:org9f2808f"
    "sec:orgb8d040b"
    "sec:org2463a01"
    "sec:orgdeaf73e"
    "sec:org19c4b78"
    "sec:org55f9c03"
    "sec:orgacff728"
    "sec:orgc41ebbd"
    "sec:orgb5bbdbb"
    "sec:orgc631535"
    "sec:org7904af6"
    "sec:org1316e46"
    "sec:org800a726"
    "sec:orgddde17a"
    "sec:org4af94b1"
    "sec:org98a5326"
    "sec:orgec63648"
    "sec:org93b0be6"
    "sec:org16aa4b3"
    "sec:org0a77174"
    "sec:org0fbc4d6"
    "sec:orgcccfe9b"
    "sec:org4a498e1"
    "sec:org6550ef9"
    "sec:org1f5fe7e"
    "sec:org53e0420"
    "sec:org104a584"
    "sec:org0a3bda7"
    "sec:orgb83e234"))
 :latex)

