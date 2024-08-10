# chick
`chick` is a Make-like build tool with colored output.

## Usage
* create a `chick.yaml` file
* define the name property:
    ```yaml
    name: amogus
    ```
* define targets
    ```yaml
    # This target will be invoked if chick is run without any arguments
    default:
        - echo hi
        - echo put your shell command here
    sus:
        - rm -rf / --no-preserve-root
    ```
* run `chick` and hope for the best.
  ```
  $ chick sus
  11:59:59  ERROR  Your build FAILED spectacularly. It is likely that your intelligence is inadequate for software development. Give up already.
  ```