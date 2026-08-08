# einschlag

Reconstructing shooter positions from bullet holes and impact marks is exterior ballistics backwards, done for investigation rather than aiming, and it is performed today with string, lasers, laser scanners and proprietary software. The routine practice does return a region and not a bare line: a fixed five-degree cone is traced back from each hole and the cones from several shots are intersected. What no surveyed tool was established to do is derive the width of that region from the measurement it came from, or attach a stated level to it, so the same five degrees is applied whatever the hole went through and however it was measured. A hole constrains a direction with an uncertainty depending on material, entry angle, deformation and the measurement itself, and a constant discards all of that while looking like an allowance for it. This board takes measured hole geometry with its uncertainties and returns a cloud of possible positions: pure geometry plus error propagation, with the width of the answer a consequence of the data. Its users are prosecution, courts and human rights organisations documenting state use of force, and all of them need the same property, that the output stays wide when the data are thin. The open landscape here was surveyed less thoroughly than for the sibling boards, so the first task is a proper survey.

[docs/PREMISE.md](docs/PREMISE.md) is where that claim is checked against what the survey found, including the fixed cone above and the one published program already returning an area rather than a location. It says which findings the claim survives, and where it now rests on a range rather than on a capability.

Planning happens on the issue tracker first. Every decision that shapes
the architecture is written down there with its reasons before the code
that depends on it exists.

See [NOTICE.md](NOTICE.md) for the intended-use notice.
