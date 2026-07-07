# BergCalc

The goal of this project is to create a fully open-source and replicable graphing calculator as a learning/teaching tool and  as a functional alternative to the likes of the TI-84. The project is currently in the software only phase, but eventually the goal is to have a no-std version of the rust library running with graphics on a Raspberry Pi Pico. 

There are 3 seperate projects contained in this repository:
- Decimal-math, a fully custom 32 digit precision decimal library. It is very buggy right now, not sure if this will be used in the final product. It is in `decimal-math/`
- A python-based calculator, stored in `python/`, this is the python prototype of the calculator source code
- A rust-based calculator, stored in `rust-calculator`/, this is the actual codebase for the eventual finished product, still very unfinished but making good progress.

# TODO (rust port)

- [x] Basic rust parser
- [x] Basic rust evaluator
- [x] Symbolic differentiation (Done in python)
- [ ] Integration (Gauss-Kronrod) (Done in python)
- [x] Expression solver using Newtons method
- [ ] Port parser/evaluator to use decimal-math library

## TODO: For embedded device

- [x] Port rust code to no-std and test on rp2350
- [ ] Start work on calculator GUI with embedded_graphics
- [ ] Basic evaluator GUI
- [ ] Graphing GUI
- [ ] Numeric solver GUI
- [ ] USB keyboard input
- [ ] Button matrix input
