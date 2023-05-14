# Emacs

## Alternative emacs configuration

To switch between two different emacs configuration, either symlink-ing
.emacs.d/ folder to different configuration or just tell emacs to load
with **supposed** init file. It comes handy to switch between
**vanilla** emacs (custom conf) and installed **doom** emacs.

## Directory structure

.my\_emacs.d - \>- init.el \>- elpa (to install packages from elpa)

.init.el:

``` 
(defvar emacs_home (expand-file-name "~/.my_emacs.d/"))
;; set new directory for emacs 
(setq user-emacs_directoy emacs_home)
;; set new package directory 
(setq package-user-dir (concat emacs_home "elpa"))
(add-to-list 'load-path (concat emacs_home "elpa"))

(set-face-attribute 'default nil :font "Fira Code-15")

(add-hook 'c-mode-common-hook
      ( lambda()
        (c-set-style 'linux)))

;; clean working environment 
(tool-bar-mode -1)
(menu-bar-mode -1)

;; load installed package
(package-initialize) 
;; like ivy-mode 
(ivy-mode)

;; mode to convert from org -> github markdown using pandoc

(add-hook 'org-mode-hook
      (lambda () 
      (defun get_default_or_supplied (out-file)
        (if (eq (length out-file) 0)
        (concat (file-name-directory buffer-file-name) (format "%s.md" (file-name-base buffer-file-name)))
          (format "%s.md" out-file)))



      (defun convert-to-gfm (out-file)
        (interactive (list
              (read-string (format "Output file [default : %s.md] : " (file-name-base buffer-file-name)))))
        (let ((file_wrote (get_default_or_supplied out-file)))
          (shell-command (format "pandoc --from=org --to=gfm %s > %s" buffer-file-name file_wrote))
          (message "Saved to >> %s" file_wrote)))
      )             
)


```

# Bash script to initialize with new configuration

Loads emacs using \`memacs\` with above configuration files. This is not
same as using init file and restriction applies. For example,

  - hook-after-init-mode won't work.
  - **save options** from menu bar won't work due to -q flag

<!-- end list -->

    MEMACS_HOME='~/.my_emacs.d'
    alias memacs='emacs -q -l ${MEMACS_HOME}/init.el'
