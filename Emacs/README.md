
# Table of Contents

1.  [Emacs](#org2b84acc)
    1.  [Alternative emacs configuration](#org2aaab43)
    2.  [Directory structure](#org6fe9602)
2.  [Bash script to initialize with new configuration](#orge76969f)



<a id="org2b84acc"></a>

# Emacs


<a id="org2aaab43"></a>

## Alternative emacs configuration

To switch between two different emacs configuration, either symlink-ing .emacs.d/ folder to different configuration or just tell emacs to load with **supposed** init file.
It comes handy to switch between **vanilla** emacs (custom conf) and installed **doom** emacs. 


<a id="org6fe9602"></a>

## Directory structure

.my<sub>emacs.d</sub> -
	    >- init.el
	    >- elpa (to install packages from elpa)

.init.el:

    (defvar emacs_home (expand-file-name "~/.my_emacs.d/"))
    ;; set new directory for emacs 
    (setq user-emacs_directoy emacs_home)
    ;; set new package directory 
    (setq package-user-dir (concat emacs_home "elpa"))
    (add-to-list 'load-path (concat emacs_home "elpa"))
    
    (set-face-attribute 'default nil :font "Fira Code-18")
    
    (add-hook 'c-mode-common-hook
    	  ( lambda()
    	    (c-set-style 'linux)))
    
    ;; clean working environment 
    (tool-bar-mode -1)
    (menu-bar-mode -1)
    
    ;; load installed package
    (package-initialize) 
    ;; like ivy-mode 
    ;; (ivy-mode)


<a id="orge76969f"></a>

# Bash script to initialize with new configuration

Loads emacs using \`memacs\` with above configuration files. 
This is not same as using init file and restriction applies. 
For example, 

-   hook-after-init-mode won't work.
-   **save options** from menu bar won't work due to -q flag

    MEMACS_HOME='~/.my_emacs.d'
    alias memacs='emacs -q -l ${MEMACS_HOME}/init.el'

