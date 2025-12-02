# Makefile for ProveIt XeTeX Documentation
#
# Usage:
#   make          - Build the PDF (runs xelatex twice for cross-references)
#   make quick    - Quick build (single pass, no index)
#   make full     - Full build with index
#   make clean    - Remove auxiliary files
#   make distclean - Remove all generated files including PDF
#   make watch    - Watch for changes and rebuild (requires fswatch)
#   make check    - Check for common LaTeX issues

# Configuration
MAIN = proveit-manual
LATEX = xelatex
LATEXFLAGS = -interaction=nonstopmode -halt-on-error -file-line-error
MAKEINDEX = makeindex

# Source files
TEX_FILES = $(wildcard *.tex)
BIB_FILES = $(wildcard *.bib)
STY_FILES = $(wildcard *.sty)

# Output files
PDF = $(MAIN).pdf
AUX_EXTS = aux log out toc lof lot idx ind ilg bbl blg nav snm vrb fls fdb_latexmk synctex.gz

# Colors for terminal output
CYAN = \033[0;36m
GREEN = \033[0;32m
YELLOW = \033[0;33m
RED = \033[0;31m
NC = \033[0m # No Color

.PHONY: all quick full clean distclean watch check help

# Default target
all: $(PDF)
	@echo "$(GREEN)Build complete: $(PDF)$(NC)"

# Quick build (single pass)
quick: $(TEX_FILES)
	@echo "$(CYAN)Running quick build (single pass)...$(NC)"
	$(LATEX) $(LATEXFLAGS) $(MAIN).tex
	@echo "$(YELLOW)Note: Cross-references may not be resolved. Run 'make' for full build.$(NC)"

# Standard build (two passes for cross-references)
$(PDF): $(TEX_FILES) $(BIB_FILES) $(STY_FILES)
	@echo "$(CYAN)Running XeLaTeX (pass 1/2)...$(NC)"
	$(LATEX) $(LATEXFLAGS) $(MAIN).tex
	@echo "$(CYAN)Running XeLaTeX (pass 2/2)...$(NC)"
	$(LATEX) $(LATEXFLAGS) $(MAIN).tex
	@echo "$(GREEN)Build complete: $(PDF)$(NC)"

# Full build with index
full: $(TEX_FILES) $(BIB_FILES) $(STY_FILES)
	@echo "$(CYAN)Running XeLaTeX (pass 1/3)...$(NC)"
	$(LATEX) $(LATEXFLAGS) $(MAIN).tex
	@if [ -f $(MAIN).idx ]; then \
		echo "$(CYAN)Building index...$(NC)"; \
		$(MAKEINDEX) $(MAIN).idx; \
	fi
	@echo "$(CYAN)Running XeLaTeX (pass 2/3)...$(NC)"
	$(LATEX) $(LATEXFLAGS) $(MAIN).tex
	@echo "$(CYAN)Running XeLaTeX (pass 3/3)...$(NC)"
	$(LATEX) $(LATEXFLAGS) $(MAIN).tex
	@echo "$(GREEN)Full build complete: $(PDF)$(NC)"

# Clean auxiliary files
clean:
	@echo "$(CYAN)Cleaning auxiliary files...$(NC)"
	@for ext in $(AUX_EXTS); do \
		rm -f $(MAIN).$$ext; \
	done
	@echo "$(GREEN)Clean complete.$(NC)"

# Clean everything including PDF
distclean: clean
	@echo "$(CYAN)Removing PDF...$(NC)"
	@rm -f $(PDF)
	@echo "$(GREEN)Distclean complete.$(NC)"

# Watch for changes (requires fswatch on macOS or inotifywait on Linux)
watch:
	@echo "$(CYAN)Watching for changes... (Ctrl+C to stop)$(NC)"
	@if command -v fswatch >/dev/null 2>&1; then \
		fswatch -o $(TEX_FILES) | xargs -n1 -I{} make quick; \
	elif command -v inotifywait >/dev/null 2>&1; then \
		while true; do \
			inotifywait -e modify $(TEX_FILES); \
			make quick; \
		done; \
	else \
		echo "$(RED)Error: Neither fswatch nor inotifywait found.$(NC)"; \
		echo "Install fswatch (macOS: brew install fswatch) or inotify-tools (Linux: apt install inotify-tools)"; \
		exit 1; \
	fi

# Check for common issues
check:
	@echo "$(CYAN)Checking for common LaTeX issues...$(NC)"
	@echo ""
	@echo "Checking for undefined references..."
	@grep -n "undefined" $(MAIN).log 2>/dev/null || echo "  $(GREEN)No undefined references found.$(NC)"
	@echo ""
	@echo "Checking for overfull boxes..."
	@grep -n "Overfull" $(MAIN).log 2>/dev/null || echo "  $(GREEN)No overfull boxes found.$(NC)"
	@echo ""
	@echo "Checking for underfull boxes..."
	@grep -n "Underfull" $(MAIN).log 2>/dev/null || echo "  $(GREEN)No underfull boxes found.$(NC)"
	@echo ""
	@echo "Checking for warnings..."
	@grep -c "Warning" $(MAIN).log 2>/dev/null || echo "  $(GREEN)No warnings found.$(NC)"

# Help
help:
	@echo "$(CYAN)ProveIt Documentation Build System$(NC)"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  $(GREEN)all$(NC)        - Build the PDF (default, runs twice for cross-refs)"
	@echo "  $(GREEN)quick$(NC)      - Quick build (single pass, cross-refs may be broken)"
	@echo "  $(GREEN)full$(NC)       - Full build with index (runs three times)"
	@echo "  $(GREEN)clean$(NC)      - Remove auxiliary files"
	@echo "  $(GREEN)distclean$(NC)  - Remove all generated files including PDF"
	@echo "  $(GREEN)watch$(NC)      - Watch for changes and rebuild automatically"
	@echo "  $(GREEN)check$(NC)      - Check for common LaTeX issues in log"
	@echo "  $(GREEN)help$(NC)       - Show this help message"
	@echo ""
	@echo "Requirements:"
	@echo "  - XeLaTeX (part of TeX Live or MiKTeX)"
	@echo "  - Latin Modern fonts"
	@echo "  - TikZ and related packages"
