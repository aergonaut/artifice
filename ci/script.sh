# This script takes care of testing your crate

set -ex

# TODO This is the "test phase", tweak it as you see fit
main() {
    cross build --target $TARGET
    cross build --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test --target $TARGET
    cross test --target $TARGET --release

    # Running the binary will generate the help text and this causes the program
    # to exit with code 1, which Travis interprets as a failure.
    cross run --target $TARGET || echo "good"
    cross run --target $TARGET --release || echo "good"
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
