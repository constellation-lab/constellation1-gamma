vesting.md

Vesting
Opyn Cosm Wasm supports vesting for options. Vesting allows the owner of an option to gradually release the option to another address over time.

To vest an option, the owner must specify the vesting period and the vesting schedule. The vesting period is the amount of time it will take for the option to be fully vested. The vesting schedule is a list of dates at which the option will be released to the other address.

For example, to vest an option over a period of 1 year, with the option being released to the other address on the 1st, 3rd, 6th, 9th, and 12th month, the vesting period would be 1 year and the vesting schedule would be [1, 3, 6, 9, 12].

Once an option has been vested, the other address will be able to exercise the option.

