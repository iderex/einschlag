# einschlag

Reconstructing shooter positions from bullet holes and impact marks is exterior ballistics backwards, done for investigation rather than aiming, and it is performed today with string, lasers and proprietary software whose output is typically a line rather than a probability distribution. A hole constrains a direction with an uncertainty depending on material, entry angle, deformation and the measurement itself, and a single line discards all of it while presenting a reconstruction as a fact. This board takes measured hole geometry with its uncertainties and returns a cloud of possible positions: pure geometry plus error propagation. Its users are prosecution, courts and human rights organisations documenting state use of force, and all of them need the same property, that the output stays wide when the data are thin. The open landscape here was surveyed less thoroughly than for the sibling boards, so the first task is a proper survey.

Planning happens on the issue tracker first. Every decision that shapes
the architecture is written down there with its reasons before the code
that depends on it exists.

See [NOTICE.md](NOTICE.md) for the intended-use notice.
