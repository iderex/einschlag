# Full text obtained for one study, 2026-08-09

`full-text-acquisition-2026-08-08.md` ends by naming one thing to try next. A
copy of Nishshanka, Shepherd and Ariyarathna, 2021,
[10.1111/1556-4029.14717](https://doi.org/10.1111/1556-4029.14717) is indexed at
the Kent Academic Repository, the fetch failed on a certificate chain before any
HTTP status existed, and that file says somebody on a route that can verify the
host should expect to get it.

It was obtained. This file carries the route and what the text says. Issue #76 is
what both are owed to.

This is a dated addition and not an edit. `ellipse-accuracy.md` records what was
read on the day it was read and is unchanged by this file. So is
`full-text-acquisition-2026-08-08.md`, whose statement that no full text had been
obtained was true of that route on that day and stays as it is.

## What was obtained

    curl -sS -m 120 --cacert bundle.pem -o kent.pdf \
      -w "http=%{http_code} type=%{content_type} size=%{size_download}\n" \
      "https://kar.kent.ac.uk/87206/1/JOFS-21-043.R1_Proof_hi.pdf"
    http=200 type=application/pdf size=2039077

    sha256sum kent.pdf
    05ca9dd57a27beb734853d14871d22a5f78c7d1086bf7c198eee88d20fd0c32b *kent.pdf

Thirty-one pages. It is the accepted manuscript sent out for peer review,
manuscript ID JOFS-21-043.R1, and not the published version, so every page and
figure number below is the manuscript's own and may not match the journal's. The
tables and figures are placed at the end of the document rather than in the text,
which is where a manuscript at that stage puts them.

The document is not added to this repository. What it says is quoted here, and
the identifiers above are what a later reader compares a copy against.

## The route, so that the next person does not repeat this one

The obstacle was a missing link in a certificate chain rather than a refusal by
the repository, and naming it is most of the value of this section.

The host serves a chain that ends at a root this machine does not trust:

    openssl s_client -connect kar.kent.ac.uk:443 -servername kar.kent.ac.uk </dev/null
    depth=2 C=US, O=ISRG, CN=Root YR
    verify error:num=19:self-signed certificate in certificate chain
    Verify return code: 19 (self-signed certificate in certificate chain)

The leaf is issued under that root and under no other. Ten certificates are sent,
and five of them do chain to `ISRG Root X1`, which is trusted here, but none of
those five issued this leaf:

    openssl s_client -connect kar.kent.ac.uk:443 -servername kar.kent.ac.uk -showcerts </dev/null 2>/dev/null > served.txt
    openssl crl2pkcs7 -nocrl -certfile served.txt | openssl pkcs7 -print_certs -noout
    subject=CN=kar.kent.ac.uk
    issuer=C=US, O=Let's Encrypt, CN=YR2
    ...
    subject=C=US, O=Let's Encrypt, CN=YR2
    issuer=C=US, O=ISRG, CN=Root YR
    subject=C=US, O=ISRG, CN=Root YR
    issuer=C=US, O=ISRG, CN=Root YR

So the leaf reaches `Root YR` and stops. That is why the failure looks like a
problem at the host and reports itself as a self-signed certificate: the last
certificate in the path the host offers signs itself.

The issuing authority publishes the same root cross-signed by `ISRG Root X1`,
which supplies the missing link. It was fetched over a connection that verifies
against the trust store already on this machine, and its signature was then
checked against `ISRG Root X1` rather than taken on trust:

    curl -sS -m 60 -o root-yr-by-x1.pem -w "http=%{http_code}\n" \
      "https://letsencrypt.org/certs/gen-y/root-yr-by-x1.pem"
    http=200

    openssl x509 -in root-yr-by-x1.pem -noout -subject -issuer
    subject=C=US, O=ISRG, CN=Root YR
    issuer=C=US, O=Internet Security Research Group, CN=ISRG Root X1

    curl -sS -m 60 -o isrgrootx1.pem -w "http=%{http_code}\n" "https://letsencrypt.org/certs/isrgrootx1.pem"
    http=200

    openssl verify -CAfile isrgrootx1.pem root-yr-by-x1.pem
    root-yr-by-x1.pem: OK

That `ISRG Root X1` is trusted by this machine, rather than merely present in a
file, was checked separately against a host the authority publishes for the
purpose:

    curl -sS -m 60 -o /dev/null -w "http=%{http_code}\n" "https://valid-isrgrootx1.letsencrypt.org/"
    http=200

The public key in the cross-signed certificate is the same key as in the root the
host serves, which is what makes it the missing link rather than a different
certificate with a similar name. `served-root-yr.pem` below is the last
certificate block of `served.txt` above, which is the self-signed `Root YR` the
host sends:

    for f in served-root-yr.pem root-yr-by-x1.pem; do
      printf "%-22s " "$f"
      openssl x509 -in $f -noout -pubkey | openssl pkey -pubin -outform DER | openssl dgst -sha256
    done
    served-root-yr.pem     SHA2-256(stdin)= 7e4e8838a8add6295de7ae3b047d3aba3488ab95db0a0aa56d897a00d8618bcf
    root-yr-by-x1.pem      SHA2-256(stdin)= 7e4e8838a8add6295de7ae3b047d3aba3488ab95db0a0aa56d897a00d8618bcf

The bundle passed to the fetch is `ISRG Root X1` followed by that cross-signed
root:

    cat isrgrootx1.pem root-yr-by-x1.pem > bundle.pem

Certificate verification was on for the fetch and was not weakened: the
`-k` and `--insecure` options were not used, and no trust store on this machine
was modified. What was supplied was a certificate the host should have sent and
did not.

The condition a route needs is therefore one sentence: a trust store carrying
`ISRG Root YR`, or the cross-signed copy of it supplied by hand as above. Which
stores carry that root was not measured here.

## What the full text says, against the fields `ellipse-accuracy.md` asks for

Its row in that file reads "abstract only" and says no error magnitude was
obtained. The full text supplies the following. The text was extracted with

    pdftotext -layout kent.pdf kent.txt

and the two figures that carry numbers are images, read from the rendered pages
rather than from extracted text, which is said again where it matters below.

**Material and projectile.** Sheet metal, 1 mm, samples of 45 by 45 cm bolted to
a target tray at the frame and not supported underneath. The ammunition is 7.62
by 39 mm standard Chinese ball with a mild steel core, copper jacket and steel
case. Average measured velocity 714.4 m/s with a standard deviation of 7.6 m/s.

**Angles tested.** From 90 degrees down in 10 degree steps to 20 degrees, with 15
degrees added because all ten shots there perforated. Set with an inclinometer of
stated precision plus or minus 0.15 degrees, rechecked before each shot. At 20
degrees the bullets showed mixed half ricochet and half perforation, and at 15
degrees all bullets ricocheted off the surface. Results from 15 to 90 degrees
were analysed.

**Sample size, and an inconsistency in how it is stated.** The method says ten
shots at each angle. The caption of Table 1 says 20 shots, the legend of Figure 7
says 10 shots, and the text discussing hole diameter at 90 degrees says "seven out
of 20 bullet holes". The manuscript does not reconcile the two numbers and no
reading of it decides which applies to which figure.

**The error of the ellipse method, tied to the angle.** This is the field the
table has been carrying as not obtained for every study in it. Figure 8, on page
28 of the manuscript, plots the mean difference between the estimated and the
known angle of incidence against the known angle, for three methods, with error
bars. Its caption reads:

> A comparison between the mean differences of the estimated angles of incidences
> using three methods (As the incident angles between 5 and 40 degrees had
> inconsistent results with the fragmentation and complex behaviour, the angles
> could not be measured using probe method for the evaluation)

The ellipse estimates were made with CloudCompare, cited by the manuscript as its
reference 15.

**What the manuscript concludes about the ellipse method.** Two sentences, quoted
because they are the finding rather than an inference from the plot:

> The new method for estimating trajectories based on the size of the bullet holes
> has proven to have the most accurate estimation of the incident angle of AK
> bullets on sheet metal within the accepted error margin of 5 degrees in bullet
> trajectory determinations. The results further highlight that the ellipse method
> is not a reliable method for this bullet and target combination.

and, in the conclusions:

> The comparative results from the trajectory rod method and ellipse method also
> highlight that the ellipse method is not viable for this bullet target
> combination and that the probing method provides the most accurate results.

**A second error the manuscript measures, which is not the ellipse relation.**
Bullets deviate from their incoming trajectory on the way through. The deviation
is around 1 degree between 40 and 90 degrees and rises to 5 degrees as the angle
falls from 40 to 20 degrees, where the core perforates and the jacket ricochets.
The manuscript states that the trajectory rod method is not recommended on 1 mm
sheet metal below 40 degrees for that reason. This bears on the model boundary
rather than on the ellipse relation, and it is a measured figure where
`methods.md` records none.

**Table 1**, page 18 of the manuscript, is the study's own estimator rather than
the ellipse method: the average full length of the impact mark against the angle
of approach, with a standard deviation per angle. It is read from the rendered
page because the table is an image.

| Angle of approach, degrees | Average full length of the impact mark, mm | Standard deviation, mm |
| --- | --- | --- |
| 90 | 7.63 | 0.24 |
| 80 | 7.78 | 0.22 |
| 70 | 7.96 | 0.26 |
| 60 | 8.87 | 0.31 |
| 50 | 9.98 | 0.6 |
| 40 | 12.23 | 0.44 |
| 30 | 17.71 | 0.55 |
| 20 | 27.99 | 0.82 |
| 15 | 37.96 | 1.56 |

The row at 15 degrees is the one where all bullets ricocheted rather than
perforated, and the table says so in its own remarks column.

## What Figure 8 gives, and the precision it does not have

The manuscript does not tabulate Figure 8. Its values exist in this document only
as plotted points, so a per-angle number can be had from it only by reading a
chart. The readings below are taken from the figure rendered at eight times scale
against gridlines two degrees apart. They are figure readings and are not
measurements this project may quote as though the study had printed them.

| Known angle, degrees | Ellipse method mean difference, degrees, read from the plot |
| --- | --- |
| 15 | about -3 |
| 20 | about -3 |
| 30 | about -0.5 |
| 40 | about -6 |
| 50 | about -4.5 |
| 60 | about -4 |
| 70 | about -2 |
| 80 | about 0 |
| 90 | about +5 |

The shape carries more than any single reading. The ellipse method underestimates
at every tested angle from 15 to 80 degrees, is furthest out at 40 degrees, and
crosses to a positive difference of about 5 degrees at 90 degrees, which is the
angle at which the perforation is circular and the relation has no axis ratio to
work from. The plotted error bars at 40 and 50 degrees reach past -8 degrees. The
worst behaviour is therefore not at the shallow end.

**Whether a value read off a chart may source a material row is not decided
here.** Issue #31 asks for a table that refuses any material it has no source for,
and this is the first study in the survey where the question is live rather than
academic. Reading it either way is a decision that belongs to #31.

## What this changes, and what it does not

It changes one row's evidence and no conclusion in this repository.

`ellipse-accuracy.md` says that by the test of a measured error tied to a stated
angle range, no material in it has a usable figure. That sentence was true of what
had been read when it was written and this file does not edit it. What is now in
this repository's holdings is one study, for one substrate and one projectile,
whose per-angle error for the ellipse method is published as a chart and whose
own conclusion is that the method is not viable for that combination.

A negative result for the ellipse method on 1 mm sheet metal is a figure and not
an absence, and it is the kind of figure a material table has to be able to carry.
A table that can only hold materials the method works on would be a table that
describes the method rather than the world.

Nothing here reaches the other eight studies. Nothing here reaches any other
material: wood, plywood, float glass, laminated glass, plastics and composites,
masonry, painted vehicle panels, profiled sheet, textiles and soft tissue are
exactly as unsourced as they were.

For issue #51, which asks for the published measured cases to be reproduced, this
manuscript gives known angles, an estimator with per-angle means and standard
deviations, and a plotted comparison. It does not give the per-shot axis
measurements the ellipse method consumes, so the ellipse results in Figure 8
cannot be recomputed from what it prints. Whether Table 1 alone is enough to
reproduce anything worth reproducing belongs to #51.

## What is still owed on issue #76

Eight of the nine studies. `full-text-acquisition-2026-08-08.md` records eight as
closed at every open-access index probed, and this route does not reach a study
that has no repository copy to fetch.

The three human routes named in the issue, interlibrary loan, an institutional
subscription and writing to the authors, were not tried and are not requests this
route makes. They remain the routes most likely to work for the remaining eight.

**It is still not established that those eight cannot be obtained.** One
certificate chain turned out to be the whole obstacle for the ninth, which is a
reason to be slower rather than quicker to write a study off as unobtainable.
