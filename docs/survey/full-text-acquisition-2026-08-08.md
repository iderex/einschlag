# Full-text acquisition attempt, 2026-08-08

`ellipse-accuracy.md` records that no full text was obtained for any study in it,
that the route it used reaches Crossref, Europe PMC, PubMed and Semantic Scholar
and is refused by Wiley, ScienceDirect and Taylor and Francis, and that the
per-angle error tables those studies carry are exactly the numbers this project
needs. Issue #76 asks for those texts to be obtained, or for it to be established
that they cannot be, with the route written down either way.

This is a dated addition and not an edit. `ellipse-accuracy.md` records what was
read on the day it was read and stays as it is. Where a full text is obtained
later, it belongs in a file like this one and the material table in issue #31
sources from there.

## What this attempt added to the earlier route

Four indexes the earlier route did not use, chosen because each aggregates
repository copies rather than publisher copies: Unpaywall, OpenAlex, Semantic
Scholar's open-access field, and the full-text URL list Europe PMC returns
alongside a record. A repository copy is an author's accepted manuscript
deposited at an institution, and it carries the tables the abstract does not.

The nine digital object identifiers probed are the three in the ellipse-method
table of `ellipse-accuracy.md` and the six in its adjacent table.

## Unpaywall

    for d in 10.1016/j.forsciint.2016.03.039 10.1111/1556-4029.14309 \
             10.1016/j.forsciint.2020.110504 10.1080/00085030.2023.2169478 \
             10.1016/j.forsciint.2021.110914 10.1111/1556-4029.15230 \
             10.1111/1556-4029.14717 10.1111/1556-4029.15431 \
             10.1111/1556-4029.14142; do
      printf "%-34s " "$d"
      curl -s "https://api.unpaywall.org/v2/$d?email=YOUR_ADDRESS" \
        | python -c "import json,sys;o=json.load(sys.stdin);print(o['oa_status'])"
    done

    10.1016/j.forsciint.2016.03.039    closed
    10.1111/1556-4029.14309            closed
    10.1016/j.forsciint.2020.110504    closed
    10.1080/00085030.2023.2169478      closed
    10.1016/j.forsciint.2021.110914    closed
    10.1111/1556-4029.15230            closed
    10.1111/1556-4029.14717            closed
    10.1111/1556-4029.15431            closed
    10.1111/1556-4029.14142            bronze

Eight of nine closed. One bronze, meaning free to read on the publisher's own
site with no open licence and no repository copy. OpenAlex, queried the same way
at `https://api.openalex.org/works/https://doi.org/<doi>`, returns the same
status for all nine and the same single bronze location.

## What the bronze location gave

    curl -sL -o nordin.pdf -w "http=%{http_code} type=%{content_type}\n" \
      "https://onlinelibrary.wiley.com/doi/pdfdirect/10.1111/1556-4029.14142"
    http=403 type=text/html; charset=UTF-8

Refused. The bytes returned are a 5567-byte HTML page and not the article.

That study is Nordin, Bominathan, Abdullah and Chang, 2020, a review of gunshot
impact marks, which `ellipse-accuracy.md` already records as carrying no error
figure. So the one document any index called free is also the one whose full text
would have added least, and it was refused anyway.

## The one repository copy that exists

Semantic Scholar and Europe PMC both name a copy of Nishshanka, Shepherd and
Ariyarathna, 2021, [10.1111/1556-4029.14717](https://doi.org/10.1111/1556-4029.14717),
in the Kent Academic Repository:

    curl -s "https://api.semanticscholar.org/graph/v1/paper/DOI:10.1111/1556-4029.14717?fields=isOpenAccess,openAccessPdf"
    {"paperId": "67feaaa71b79dc9ffdb32462b5ea823731f4e1f9", "isOpenAccess": true, "openAccessPdf": {"url": "https://kar.kent.ac.uk/87206/1/JOFS-21-043.R1_Proof_hi.pdf", "status": "GREEN", "license": null}}

**It was not obtained, and the reason is on this side rather than at the
repository.** The fetch fails before any HTTP status exists:

    curl -sS -m 60 -o /dev/null "https://kar.kent.ac.uk/87206/1/JOFS-21-043.R1_Proof_hi.pdf"
    curl: (60) schannel: SEC_E_UNTRUSTED_ROOT (0x80090325)

The same URL fails with the same class of error against the certifi bundle rather
than the system trust store, so it is not a single store being out of date:

    python -c "import certifi,ssl,urllib.request; urllib.request.urlopen('https://kar.kent.ac.uk/87206/', timeout=60, context=ssl.create_default_context(cafile=certifi.where()))"
    urllib.error.URLError: <urlopen error [SSL: CERTIFICATE_VERIFY_FAILED] certificate verify failed: self-signed certificate in certificate chain (_ssl.c:1082)>

A control host verifies on the same route in the same run, so the failure is
specific to that host and not a general interception of outbound traffic:

    python -c "import certifi,ssl,urllib.request; print(urllib.request.urlopen('https://api.openalex.org/works/https://doi.org/10.1111/1556-4029.14717', timeout=60, context=ssl.create_default_context(cafile=certifi.where())).status)"
    200

Plain HTTP redirects back to the same HTTPS URL and therefore fails the same way:

    curl -sSL -m 60 -o /dev/null -w "http=%{http_code}\n" \
      "http://kar.kent.ac.uk/87206/1/JOFS-21-043.R1_Proof_hi.pdf"
    http=302

Fetching it with certificate verification disabled was not available on this
route. That is recorded as the state rather than as a judgement about whether it
would have been the right thing to do.

**So this study is not established as unobtainable.** A copy is indexed twice, at
a named repository, and the obstacle is a certificate chain. Somebody on a route
that can verify that host, or with a browser, should expect to get it. It is the
first thing to try next, and this file exists partly so that the next person does
not repeat the four indexes above before reaching it.

## Two records that look like repository copies and are not

OpenAlex lists institutional repository locations for two studies:

- Kerkhoff, Broekhuis, Mattijssen and Riva, 2024,
  [10.1111/1556-4029.15431](https://doi.org/10.1111/1556-4029.15431), at IRIS and
  at SERVAL.
- Riva, Broekhuis, Haag, Koene and Kerkhoff, 2025,
  [10.1111/1556-4029.15697](https://doi.org/10.1111/1556-4029.15697), at the same
  two.

Both landing pages return HTTP 200 and carry no article text. What comes back is
the shell of a single-page application: the fetched bytes contain the interface
strings for an attachment access-conditions panel and no document, so the page a
person sees is assembled after the fetch this route makes. Neither location is
marked open access by OpenAlex, and the Wiley copy of the 2024 study is refused
with HTTP 403 exactly as the bronze one above.

This is the same shape `methods.md` records for its digital object identifier
resolutions, where two of three returned HTTP 200 and neither response was
article text. A 200 is not a document.

## Result, per study

| Study | DOI | Full text obtained | What stands in the way |
| --- | --- | --- | --- |
| Mattijssen and Kerkhoff, 2016 | 10.1016/j.forsciint.2016.03.039 | no | closed at every index probed; publisher refuses this route |
| Walters and Liscio, 2020 | 10.1111/1556-4029.14309 | no | closed at every index probed |
| Liscio and Imran, 2020 | 10.1016/j.forsciint.2020.110504 | no | closed at every index probed |
| Santangelo, Liscio and Nugent, 2023 | 10.1080/00085030.2023.2169478 | no | closed; not indexed in Europe PMC at all |
| Liscio and Park, 2021 | 10.1016/j.forsciint.2021.110914 | no | closed at every index probed |
| Greenwood, Paduch and Allen, 2023 | 10.1111/1556-4029.15230 | no | closed at every index probed |
| Nishshanka, Shepherd and Ariyarathna, 2021 | 10.1111/1556-4029.14717 | no | repository copy indexed twice; certificate chain does not verify on this route |
| Kerkhoff, Broekhuis, Mattijssen and Riva, 2024 | 10.1111/1556-4029.15431 | no | publisher HTTP 403; two repository landing pages carry no text |
| Nordin, Bominathan, Abdullah and Chang, 2020 | 10.1111/1556-4029.14142 | no | listed bronze; publisher HTTP 403 |

**No full text was obtained for any study.** Nothing in
`ellipse-accuracy.md` changes, and its statement that no material in it has a
measured error tied to a stated angle range still holds.

## What was not tried, and why it is named rather than omitted

Issue #76 lists interlibrary loan, an institutional subscription, an author
preprint and writing to the authors. Of those, the preprint route is the one this
attempt covers, through the four repository indexes above, and it produced the
one Kent copy. The other three are not available on this route: none of them is a
request a machine makes, and two of them need an institution this project does
not have.

They remain the routes most likely to work, and the two ordered by cost are
writing to the corresponding authors, several of whom appear on more than one of
these studies, and an interlibrary request through any institution a reader of
this file has access to.

## What this establishes, and what it does not

It establishes that these nine studies are not reachable through the open-access
aggregation indexes, which is more than `ellipse-accuracy.md` had established,
because that file's route did not include them. Eight are closed at every index.

**It does not establish that the full texts cannot be obtained.** One is sitting
in a named repository behind a certificate problem, and three human routes were
not tried at all. Saying otherwise would turn "we have no figure" into "there is
no figure", which is the distinction issue #76 was written to protect.

## What it means for the two dependent issues

Unchanged from what issue #76 states, and this file does not soften it.

Issue #31 asks for a material table that refuses any material it has no source
for. Sourced from this repository's reading today, that table refuses every
material and the first version cannot compute an uncertainty for any real hole.

Issue #51 asks for the published measured cases to be reproduced. The per-angle
error tables are what would be reproduced and none were obtained, so there is
nothing to run through the tool and compare.

Both issues are outside what this file can change. Re-planning them belongs to
them, and this file is the evidence that the re-planning is now owed rather than
waiting on an arrival.

## Where the one file named above is

`methods.md` is not on the default branch at the commit this file was written on.
It is on the branch of the open pull request that lands issues #3, #5, #7, #11,
#13, #15, #17 and #19, and it was read there at
`8134ddc19a7a957c9e94e5e959a30377089fe502`:

    git show 8134ddc19a7a957c9e94e5e959a30377089fe502:docs/survey/methods.md

The reference to it resolves once that pull request lands, and until it does it
does not. `ellipse-accuracy.md` is in the tree at this commit and is the file
this one is an addition to.
