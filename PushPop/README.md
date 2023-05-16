# Push and Pop

  - Globally store directory across multiple terminal sessions and
    retrieve them later
  - Use with xargs for better ergonomics
  - cd can't be used in conjunction with xargs (since its a shell
    builtin)

# Install instructions

  - Compile as usual
  - Optionally add the binaries to the PATH variable

<!-- end list -->

``` bash
cp ./gpush ./gpop ~/bin/
```

  - Add following to the .bashrc

<!-- end list -->

``` bash
export PATH="~/bin:$PATH" 
cdl() {
      cd $(gpop) 
}
```

cdl then can be used to behave like **popd**.
