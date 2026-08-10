# Full-text acquisition attempt, repository records, 2026-08-10

Issue #76 asks for the full texts behind `ellipse-accuracy.md` to be obtained, or
for it to be established that they cannot be, with the route written down either
way. This is the fourth dated addition under that issue and it is not an edit.
`ellipse-accuracy.md` stays as it is, and so do
`full-text-acquisition-2026-08-08.md`, `full-text-2026-08-09-nishshanka-2021.md`
and `full-text-acquisition-2026-08-10.md`.

One of the nine has been obtained, at the Kent Academic Repository, and
`full-text-2026-08-09-nishshanka-2021.md` carries it. This attempt is about the
other eight, and it changes one thing about how they were probed.

## The route asked a different question of the same responses

The three earlier attempts asked the indexes whether a free copy exists.
Unpaywall's `oa_status`, OpenAlex's single best open-access location, Semantic
Scholar's open-access field, and OpenAIRE's access rights are all answers to that
question, and all four came back with nothing for these eight.

A deposit is not the same thing as a free copy. A repository record can exist and
be closed, embargoed, or metadata only, and every field above reports it as
absent. So this attempt read the location list rather than the access status, and
asked whether a repository holds a record at all.

## What the location list holds

OpenAlex returns every location it knows for a work, not only the best open one.
Counting the ones whose source is typed as a repository:

    for d in 10.1016/j.forsciint.2016.03.039 10.1111/1556-4029.14309 \
             10.1016/j.forsciint.2020.110504 10.1080/00085030.2023.2169478 \
             10.1016/j.forsciint.2021.110914 10.1111/1556-4029.15230 \
             10.1111/1556-4029.15431 10.1111/1556-4029.14142; do
      printf '%-34s ' "$d"
      curl -s "https://api.openalex.org/works/doi:$d" | python -c "
    import json,sys
    o=json.load(sys.stdin)
    rep=[l for l in o.get('locations',[]) if ((l.get('source') or {}).get('type')=='repository')]
    print('locations=%d repository=%d %s' % (len(o.get('locations',[])), len(rep),
          [(l['source']['display_name'], l.get('version')) for l in rep]))
    "
    done

    10.1016/j.forsciint.2016.03.039    locations=2 repository=1 [('PubMed', 'publishedVersion')]
    10.1111/1556-4029.14309            locations=2 repository=1 [('PubMed', 'publishedVersion')]
    10.1016/j.forsciint.2020.110504    locations=2 repository=1 [('PubMed', 'publishedVersion')]
    10.1080/00085030.2023.2169478      locations=1 repository=0 []
    10.1016/j.forsciint.2021.110914    locations=2 repository=1 [('PubMed', 'publishedVersion')]
    10.1111/1556-4029.15230            locations=2 repository=1 [('PubMed', 'publishedVersion')]
    10.1111/1556-4029.15431            locations=4 repository=3 [('PubMed', 'publishedVersion'), ('IRIS', 'submittedVersion'), ('SERVAL (Université de Lausanne)', 'submittedVersion')]
    10.1111/1556-4029.14142            locations=2 repository=1 [('PubMed', 'publishedVersion')]

Seven of the eight have exactly one repository location and it is PubMed, which
holds a bibliographic record and an abstract rather than an article, so those
rows are the index restating what a reader already had.

The eighth is not. Kerkhoff, Broekhuis, Mattijssen and Riva 2024,
[10.1111/1556-4029.15431](https://doi.org/10.1111/1556-4029.15431), has two
further repository locations, both at the Université de Lausanne, and both
carrying a submitted version. **No earlier record in this directory names either
of them**, and the reason is the field that was read: the work's open-access
status is closed, so every route that asked about access reported no repository
copy while the location list named two.

## What that record turns out to be

The two addresses are one item. The SERVAL notice redirects into the university's
IRIS instance:

    curl -sL -o /dev/null -w "http=%{http_code} url=%{url_effective}\n" \
      "https://serval.unil.ch/notice/serval:BIB_B6108B9A0D67"
    http=200 url=https://iris.unil.ch/entities/publication/23b02376-f595-4008-bbbe-cd167fd6aaeb

The item carries the abstract, the DOI, the PubMed identifier and the Web of
Science identifier, and its files are two:

    curl -s -H "Accept: application/json" \
      "https://api.unil.ch/iris/server/api/core/bitstreams/5e166922-df30-4de7-954d-a0dcefcb281a" \
      | python -c "import json,sys; o=json.load(sys.stdin); print(o['name'], o['sizeBytes'])"
    serval_BIB_B6108B9A0D67.zip 3903

The other is `license.txt` at 161 bytes. Both refuse an unauthenticated request:

    curl -sL -o body -w "http=%{http_code} type=%{content_type} bytes=%{size_download}\n" \
      "https://api.unil.ch/iris/server/api/core/bitstreams/5e166922-df30-4de7-954d-a0dcefcb281a/content"
    http=401 type=application/json bytes=219

    cat body
    {"timestamp":"2026-08-10T08:49:45.463+00:00","status":401,"error":"Unauthorized","message":"Authentication is required","path":"/iris/server/iris/server/api/core/bitstreams/5e166922-df30-4de7-954d-a0dcefcb281a/content"}

So the deposit exists, its only candidate file is 3903 bytes, and that file is
refused. Whatever it is, an accepted manuscript of a journal article is not
3903 bytes.

This is a second shape of the thing
`full-text-acquisition-2026-08-10.md` recorded for Elsevier, and it is worth
keeping separate from it. There, a publisher endpoint returned HTTP 200 and a
document named `full-text-retrieval-response` with no text in it. Here, an index
names a repository copy and the repository holds no readable manuscript behind it.
One door reports as open at the publisher and one at the aggregator, and a route
that counted either as an arrival would be wrong in the same direction both times.

The article is also the one of the eight whose subject is furthest from what
`ellipse-accuracy.md` is looking for. Its abstract, read from the item above, is
about the vertical error gravity introduces over distance and proposes threshold
distances up to which a trajectory can be modelled as a straight line. That is a
figure for the model boundary rather than a per-material, per-angle ellipse error,
so obtaining it would not move the empty material table.

## The query key was not what hid it

Every earlier probe queried by DOI. A deposit whose metadata omits the DOI is
invisible to that query, so the same eight titles were put to OpenAIRE by title
instead. The titles are long enough that the loop is written here with the title
elided and the DOI printed beside each answer, which is what the rows are keyed
by everywhere else in this directory:

    for each of the eight titles as printed by Crossref:
      curl -s "https://api.openaire.eu/search/publications?title=<title>&format=json&size=20" \
        | python -c "..."   # hit count, and any repository or PDF address in the record

    10.1016/j.forsciint.2016.03.039    title-hits=1 candidate-urls=0
    10.1111/1556-4029.14309            title-hits=1 candidate-urls=0
    10.1016/j.forsciint.2020.110504    title-hits=1 candidate-urls=0
    10.1080/00085030.2023.2169478      title-hits=1 candidate-urls=0
    10.1016/j.forsciint.2021.110914    title-hits=1 candidate-urls=0
    10.1111/1556-4029.15230            title-hits=1 candidate-urls=0
    10.1111/1556-4029.15431            title-hits=1 candidate-urls=0
    10.1111/1556-4029.14142            title-hits=1 candidate-urls=0

One record per title and no repository address on any of them, including the one
OpenAlex places at Lausanne. So the title query returns what the DOI query
returned, and what hid the Lausanne record from the earlier route was the field
read out of the response rather than the key it was looked up by.

## PubMed Central, asked directly

Seven of the eight carry a PubMed identifier. Whether any of them has a free
copy in PubMed Central is one request per identifier, and it was not asked before:

    for p in 27044032 32092785 32980717 34333194 36929024 38037231 31373699; do
      printf '%-9s ' "$p"
      curl -s "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/elink.fcgi?dbfrom=pubmed&db=pmc&id=$p&retmode=json" \
        | python -c "import json,sys; o=json.load(sys.stdin); print([db.get('linkname') for s in o['linksets'] for db in s.get('linksetdbs',[])])"
    done

    27044032  ['pubmed_pmc_refs']
    32092785  ['pubmed_pmc_refs']
    32980717  ['pubmed_pmc_refs']
    34333194  ['pubmed_pmc_refs']
    36929024  ['pubmed_pmc_refs']
    38037231  ['pubmed_pmc_refs']
    31373699  ['pubmed_pmc_refs']

`pubmed_pmc_refs` is the set of articles in PubMed Central that cite the record,
which is the opposite direction from a copy of it. The link that would name a
copy, `pubmed_pmc`, is absent from all seven. The eighth,
10.1080/00085030.2023.2169478, has no PubMed identifier in the location list
above, so the question does not arise for it.

## No full text was obtained

For any of the eight. Nothing in `ellipse-accuracy.md` changes and its statement
that no material has a measured error tied to a stated angle range still holds.

## What was tried and did not answer

CORE is still untried rather than tried and failed, and for the same reason as
before:

    curl -s -m 25 -o /dev/null -w "http=%{http_code}\n" "https://api.core.ac.uk/v3/search/works"
    http=000

Its web search refuses this route rather than not answering, which is a different
state from the one above and is recorded so the two are not merged later:

    curl -sL -m 40 -o /dev/null -w "http=%{http_code} url=%{url_effective}\n" "https://core.ac.uk/search?q=test"
    http=403 url=https://core.ac.uk/search/?q=test

Interlibrary loan, an institutional subscription and writing to the authors are
the three routes issue #76 names that are not requests this route can make, and
they remain untried. That is unchanged and this file does not soften it.

## What is owed after this

The eight full texts, and those three routes. Unchanged.

One thing is added to what a later attempt should do rather than to what it
should stop doing. Where an index exposes a list of locations, read the list and
not the access status, and then ask the repository what it actually holds. That
is measured for OpenAlex here and nowhere else: whether Unpaywall, Semantic
Scholar or OpenAIRE expose a location that their access-status field reports as
absent was not evaluated on this route.

## What this file is not

It is not a claim that the eight cannot be obtained. It records that one
repository record existed and went unnamed for three attempts because of which
field was read, which is a reason to be slower rather than quicker to write any
of them off. It is also not a claim that no other such record exists: the check
above reads what OpenAlex knows, and an index not knowing of a deposit is not the
deposit's absence.
