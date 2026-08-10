# Full-text acquisition attempt, 2026-08-10

Issue #76 asks for the full texts behind `ellipse-accuracy.md` to be obtained, or
for it to be established that they cannot be, with the route written down either
way so that the next person does not repeat it. This is the third dated addition
under that issue and it is not an edit. `ellipse-accuracy.md` records what was
read on the day it was read and stays as it is, and so do
`full-text-acquisition-2026-08-08.md` and `full-text-2026-08-09-nishshanka-2021.md`.

One of the nine has been obtained, at the Kent Academic Repository, and
`full-text-2026-08-09-nishshanka-2021.md` carries it. This attempt is about the
other eight.

## What this attempt added to the earlier route

Three things, and the third is the one worth carrying forward.

**OpenAIRE**, which the earlier route did not use. It aggregates repository
copies from institutional and national repositories across Europe and beyond, and
it is a different population from Unpaywall, OpenAlex and Semantic Scholar, which
the attempt of 2026-08-08 probed.

**The link array Crossref returns beside a record**, which the earlier route
reached Crossref without reading. It names the endpoint each publisher exposes
for text and mining, per article, which turns "the publisher refuses" into a
named thing a subscription would unlock.

**Those endpoints, requested without credentials.** What each one does when
nobody is entitled to it is the part that was not known before, and one of the
two answers is not the answer a status code alone would give.

## OpenAIRE

    for d in 10.1016/j.forsciint.2016.03.039 10.1111/1556-4029.14309 \
             10.1016/j.forsciint.2020.110504 10.1080/00085030.2023.2169478 \
             10.1016/j.forsciint.2021.110914 10.1111/1556-4029.15230 \
             10.1111/1556-4029.15431 10.1111/1556-4029.14142; do
      printf '%-34s ' "$d"
      curl -s "https://api.openaire.eu/search/publications?doi=$d&format=json" \
        | python -c "..."   # total, access rights, and any full-text URL
    done

    10.1016/j.forsciint.2016.03.039    total=1 rights=['Closed Access', 'unspecified'] urls=0
    10.1111/1556-4029.14309            total=1 rights=['Closed Access', 'unspecified'] urls=0
    10.1016/j.forsciint.2020.110504    total=1 rights=['Closed Access', 'unspecified'] urls=0
    10.1080/00085030.2023.2169478      total=1 rights=['unspecified'] urls=0
    10.1016/j.forsciint.2021.110914    total=1 rights=['Closed Access', 'unspecified'] urls=0
    10.1111/1556-4029.15230            total=1 rights=['Closed Access', 'unspecified'] urls=0
    10.1111/1556-4029.15431            total=1 rights=['Closed Access', 'unspecified'] urls=0
    10.1111/1556-4029.14142            total=1 rights=['Closed Access', 'unspecified'] urls=0

Each of the eight is known to OpenAIRE and none of them has a repository copy
there. The eighth line is the one whose Unpaywall status was bronze rather than
closed, and bronze is free to read on the publisher's own site rather than
deposited anywhere, which is what a repository index having nothing for it means.

That is a fourth index agreeing with the three the earlier attempt used, and the
agreement is worth something precisely because OpenAIRE draws on a different
population of repositories. It is not proof that no accepted manuscript exists.
It is four independent indexes not knowing of one.

## The endpoints Crossref names

    for d in <the same eight>; do
      printf '%-34s ' "$d"
      curl -s "https://api.crossref.org/works/$d" \
        | python -c "..."   # the link array, by content type and intended use
    done

Reduced to the distinct shapes, because eight full listings say the same three
things:

    10.1016/j.forsciint.2016.03.039
      text/xml    text-mining  https://api.elsevier.com/content/article/PII:S0379073816301268?httpAccept=text/xml
      text/plain  text-mining  https://api.elsevier.com/content/article/PII:S0379073816301268?httpAccept=text/plain
    10.1111/1556-4029.14309
      application/pdf  text-mining  https://api.wiley.com/onlinelibrary/tdm/v1/articles/10.1111%2F1556-4029.14309
      application/pdf  text-mining  https://onlinelibrary.wiley.com/doi/pdf/10.1111/1556-4029.14309
      application/xml  text-mining  https://onlinelibrary.wiley.com/doi/full-xml/10.1111/1556-4029.14309
    10.1080/00085030.2023.2169478
      unspecified  similarity-checking  https://www.tandfonline.com/doi/pdf/10.1080/00085030.2023.2169478

The three Elsevier articles carry the first shape, four of the Wiley articles
carry the second in whole or in part, and the one Taylor and Francis article
carries only a similarity-checking link, which is an endpoint for a plagiarism
service rather than for a reader.

## What those endpoints do without credentials

    curl -sL -o /dev/null -w "http=%{http_code} type=%{content_type} bytes=%{size_download}\n" \
      "https://api.wiley.com/onlinelibrary/tdm/v1/articles/10.1111%2F1556-4029.14309"
    http=400 type=text/plain bytes=0

    curl -s -D - -o /dev/null \
      "https://api.wiley.com/onlinelibrary/tdm/v1/articles/10.1111%2F1556-4029.14309"
    HTTP/1.1 400 No TDM Client Token was found in the request

Wiley states what is missing by name. A client token for text and data mining is
a credential an institution holds, and the endpoint is otherwise the article.

    curl -sL -o /dev/null -w "http=%{http_code} type=%{content_type} bytes=%{size_download}\n" \
      "https://onlinelibrary.wiley.com/doi/full-xml/10.1111/1556-4029.15230"
    http=403 type=text/html; charset=UTF-8 bytes=5564

    curl -sL -o /dev/null -w "http=%{http_code} type=%{content_type} bytes=%{size_download}\n" \
      "https://www.tandfonline.com/doi/pdf/10.1080/00085030.2023.2169478"
    http=403 type=text/html; charset=UTF-8 bytes=5542

Both refused, and both return an HTML page of about five and a half kilobytes
rather than an article, which is the same shape `ellipse-accuracy.md` records for
the publisher pages.

Elsevier is the one that does not refuse, and the answer is worse than a refusal.

    curl -s -m 60 -o els1.xml -w "http=%{http_code} bytes=%{size_download}\n" \
      "https://api.elsevier.com/content/article/PII:S0379073816301268"
    http=200 bytes=1828

The request without an API key returns HTTP 200 and an XML document whose root
element is `full-text-retrieval-response`. It carries 1828 bytes, and what is in
them is the title, the journal, the identifiers, the cover date and

    <openaccess>0</openaccess>
    <openaccessArticle>false</openaccessArticle>

There is no `originalText` element, no section, no paragraph and no abstract. The
document is a metadata stub wearing the name of a full-text response.

**A route that checks the status code and the root element records that one as a
success.** It is the only one of the three publishers that returns 200 without
entitlement, and it is the one that gives the least. That is the finding this
file exists for: not that the article is behind a subscription, which was already
recorded, but that this particular door reports as open and is not.

## No full text was obtained

For any of the eight. Nothing in `ellipse-accuracy.md` changes and its statement
that no material has a measured error tied to a stated angle range still holds.

## What is now named that was not named before

The institutional-subscription route, which issue #76 lists and which is not a
request a fetching route makes, has a target rather than a direction. For the
four Wiley articles it is a text and data mining client token, refused by name in
the response above. For the three Elsevier articles it is an API key with
entitlement to the full-text endpoint already named in the Crossref link array.
For the one Taylor and Francis article, Crossref names no reader endpoint at all,
only a similarity-checking one, so that article has no machine route even with a
credential and interlibrary loan or the authors are what is left for it.

## What was not tried, and why

CORE, which is the remaining large aggregator of repository copies, was not
tried. Its version 3 interface did not answer from this route:

    curl -s -m 20 -o /dev/null -w "http=%{http_code}\n" "https://api.core.ac.uk/v3/search/works"
    http=000

A zero status code is no response rather than a refusal, so CORE is untried and
not exhausted. It also requires a registered key, which is the same class of
thing as the two credentials above.

BASE requires registration for its interface and was not tried for that reason.

Interlibrary loan, an institutional subscription and writing to the authors are
the three routes issue #76 names that are not requests this route can make, and
they remain untried. That is unchanged and this file does not soften it.

## What it means for the two dependent issues

Unchanged, and stated again only because a reader arriving at this file first
should not have to infer it.

Issue #31 asks for a material table that refuses any material it has no source
for. Sourced from what this repository has read, that table refuses every
material.

Issue #51 asks for the published measured cases to be reproduced. Seven of the
nine studies have no full text here, and the one that was obtained plots its
per-angle comparison rather than tabulating it, which
`full-text-2026-08-09-nishshanka-2021.md` records and labels as figure readings.

Both issues are outside what this file can change, and re-planning them belongs
to them.

## What this file is not

It is not a claim that the eight cannot be obtained. Four indexes not knowing of
a copy and three publishers refusing an unentitled request establish that this
route does not reach them, which is a different statement, and issue #76 exists
to keep those two apart. One certificate chain turning out to be the whole
obstacle for the ninth is the reason to keep them apart.
