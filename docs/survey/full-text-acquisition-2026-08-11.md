# Full-text acquisition attempt, the other three indexes and CORE, 2026-08-11

Issue #76 asks for the full texts behind `ellipse-accuracy.md` to be obtained, or
for it to be established that they cannot be, with the route written down either
way. This is the fifth dated addition under that issue and it is not an edit.
`ellipse-accuracy.md` stays as it is, and so do
`full-text-acquisition-2026-08-08.md`, `full-text-2026-08-09-nishshanka-2021.md`,
`full-text-acquisition-2026-08-10.md` and
`full-text-acquisition-2026-08-10-repository-records.md`.

One of the nine has been obtained and the record for it carries what it says.
This attempt is about the other eight and it does two things the earlier ones
left undone. It puts the question
`full-text-acquisition-2026-08-10-repository-records.md` closed with to the three
indexes that were not measured for it, and it reaches CORE, which every earlier
record names as untried rather than tried and failed.

No full text was obtained for any of the eight.

## The question the last record left open

That record found a repository deposit for one of the eight by reading OpenAlex's
list of locations rather than its access status, and ended by saying that whether
Unpaywall, Semantic Scholar or OpenAIRE expose a location their access-status
field reports as absent was not evaluated on that route. It is evaluated here, on
the same eight. The work that tests it is Kerkhoff, Broekhuis, Mattijssen and
Riva 2024, [10.1111/1556-4029.15431](https://doi.org/10.1111/1556-4029.15431),
because that is the one with a deposit to find.

Unpaywall answers the question in a field that is neither the access status nor a
best location, so the reading is `has_repository_copy` beside the whole
`oa_locations` list and the embargoed list:

    for d in 10.1016/j.forsciint.2016.03.039 10.1111/1556-4029.14309 \
             10.1016/j.forsciint.2020.110504 10.1080/00085030.2023.2169478 \
             10.1016/j.forsciint.2021.110914 10.1111/1556-4029.15230 \
             10.1111/1556-4029.15431 10.1111/1556-4029.14142; do
      printf '%-34s ' "$d"
      curl -s "https://api.unpaywall.org/v2/$d?email=..." | python -c "
    import json,sys
    o=json.load(sys.stdin)
    loc=(o.get('oa_locations') or [])+(o.get('oa_locations_embargoed') or [])
    print('repository_copy=%s locations=%d %s' % (o.get('has_repository_copy'), len(loc), [l.get('host_type') for l in loc]))
    "
    done

    10.1016/j.forsciint.2016.03.039    repository_copy=False locations=0 []
    10.1111/1556-4029.14309            repository_copy=False locations=0 []
    10.1016/j.forsciint.2020.110504    repository_copy=False locations=0 []
    10.1080/00085030.2023.2169478      repository_copy=False locations=0 []
    10.1016/j.forsciint.2021.110914    repository_copy=False locations=0 []
    10.1111/1556-4029.15230            repository_copy=False locations=0 []
    10.1111/1556-4029.15431            repository_copy=False locations=0 []
    10.1111/1556-4029.14142            repository_copy=False locations=1 ['publisher']

The one location any of the eight has is at a publisher, and it is the bronze
location `full-text-acquisition-2026-08-08.md` already recorded as refused with
HTTP 403. For 15431 Unpaywall states in a dedicated field that no repository copy
exists, against a deposit two indexes name below. That is a stronger negative
than an access status and it is wrong in the same direction.

Semantic Scholar exposes no location list at all on the paper endpoint. What it
has is one open-access address, which is the field the earlier route already
read:

    for d in <the same eight>; do
      printf '%-34s ' "$d"
      curl -s "https://api.semanticscholar.org/graph/v1/paper/DOI:$d?fields=isOpenAccess,openAccessPdf" | python -c "
    import json,sys
    o=json.load(sys.stdin)
    print('open_access=%s pdf=%s' % (o.get('isOpenAccess'), (o.get('openAccessPdf') or {}).get('url')))
    "
    done

    10.1016/j.forsciint.2016.03.039    open_access=False pdf=
    10.1111/1556-4029.14309            open_access=False pdf=
    10.1016/j.forsciint.2020.110504    open_access=False pdf=
    10.1080/00085030.2023.2169478      open_access=False pdf=
    10.1016/j.forsciint.2021.110914    open_access=False pdf=
    10.1111/1556-4029.15230            open_access=False pdf=
    10.1111/1556-4029.15431            open_access=False pdf=
    10.1111/1556-4029.14142            open_access=True pdf=https://onlinelibrary.wiley.com/doi/pdfdirect/10.1111/1556-4029.14142

So for this index the advice cannot be followed, and that is the answer rather
than a gap in the attempt: there is no list to read instead of the status. The
one address it returns is the publisher address already recorded as refused.

OpenAIRE does expose a list, one entry per instance, and the name of what hosts
each one:

    for d in <the same eight>; do
      printf '%-34s ' "$d"
      curl -s "https://api.openaire.eu/search/publications?doi=$d&format=json" | python -c "
    import json,sys
    def walk(o,k,out):
        if isinstance(o,dict):
            for a,b in o.items():
                walk(b,k,out) if a!=k else out.append(b)
        elif isinstance(o,list):
            for v in o: walk(v,k,out)
    o=json.load(sys.stdin); inst=[]; walk(o,'instance',inst)
    flat=[]
    for i in inst: flat.extend(i if isinstance(i,list) else [i])
    hb=[]; walk(flat,'hostedby',hb)
    names=set()
    for h in hb:
        for x in (h if isinstance(h,list) else [h]):
            if isinstance(x,dict): names.add(x.get('@name','?'))
    print('instances=%d hostedby=%s' % (len(flat), sorted(names)))
    "
    done

    10.1016/j.forsciint.2016.03.039    instances=4 hostedby=['Forensic Science International', 'Unknown Repository']
    10.1111/1556-4029.14309            instances=4 hostedby=['Journal of Forensic Sciences', 'Unknown Repository']
    10.1016/j.forsciint.2020.110504    instances=4 hostedby=['Forensic Science International', 'Unknown Repository']
    10.1080/00085030.2023.2169478      instances=1 hostedby=['Canadian Society of Forensic Science Journal']
    10.1016/j.forsciint.2021.110914    instances=4 hostedby=['Forensic Science International', 'Unknown Repository']
    10.1111/1556-4029.15230            instances=4 hostedby=['Journal of Forensic Sciences']
    10.1111/1556-4029.15431            instances=4 hostedby=['Journal of Forensic Sciences']
    10.1111/1556-4029.14142            instances=4 hostedby=['Journal of Forensic Sciences', 'Unknown Repository']

The list names the journal, and on five of the eight an entry OpenAIRE itself
labels `Unknown Repository`. What that entry is is not asserted here, because the
response does not say. What matters for the question is the row for 15431: its
instance list names the journal and nothing else, so reading the list rather than
the access status does not surface the Lausanne deposit at this index either.

The answer to the question the last record left open is therefore negative for
all three. None of them exposes the deposit that OpenAlex's location list named,
and one of them states its absence in a field of its own. The route that found it
is not redundant with the other three, and reading a list instead of a status is
worth doing at each index separately rather than once.

## CORE now answers, and it was recorded as not answering

Every earlier record keeps CORE as untried rather than exhausted, and the reason
was that the endpoint returned nothing at all. It returns something today. The
address in those records redirects, and the redirect target answers without a
key:

    curl -s -m 30 -o /dev/null -w "http=%{http_code} redirect=%{redirect_url}\n" "https://api.core.ac.uk/v3/search/works"
    http=301 redirect=https://api.core.ac.uk/v3/search/works/

    curl -sL -m 40 -o /dev/null -w "followed http=%{http_code} type=%{content_type}\n" "https://api.core.ac.uk/v3/search/works/"
    followed http=200 type=application/json

Whether the earlier `http=000` was the missing trailing slash, a transient
failure or a change at CORE is not established here, and the earlier records are
not edited. What is established is that the request the earlier route could not
make can be made now.

The web search still refuses this route, which is the state the last record asked
to be kept separate from the one above, and it is unchanged:

    curl -sL -m 40 -o /dev/null -w "web http=%{http_code} url=%{url_effective}\n" "https://core.ac.uk/search?q=test"
    web http=403 url=https://core.ac.uk/search/?q=test

Asked for the eight by identifier:

    for d in <the same eight>; do
      printf '%-34s ' "$d"
      curl -s "https://api.core.ac.uk/v3/search/works/?q=doi:%22$d%22" | python -c "
    import json,sys
    o=json.load(sys.stdin)
    r=o.get('results') or []
    print('hits=%s %s' % (o.get('totalHits'),
      [([p['name'] for p in (x.get('dataProviders') or [])], x.get('downloadUrl'), x.get('sourceFulltextUrls'), x.get('fullText')) for x in r]))
    "
    done

    10.1016/j.forsciint.2016.03.039    hits=0 []
    10.1111/1556-4029.14309            hits=1 [(['Crossref'], '', [], 'Not available for public API users.')]
    10.1016/j.forsciint.2020.110504    hits=0 []
    10.1080/00085030.2023.2169478      hits=0 []
    10.1016/j.forsciint.2021.110914    hits=0 []
    10.1111/1556-4029.15230            hits=0 []
    10.1111/1556-4029.15431            hits=1 [(['UNIL IRIS | Institutional Research Information System'], '', [], 'Not available for public API users.')]
    10.1111/1556-4029.14142            hits=1 [(['Crossref'], '', [], 'Not available for public API users.')]

Five of the eight are not in CORE at all. Two are there by way of Crossref
metadata, which is a bibliographic record and not a deposit. The third is 15431,
and its provider is the University of Lausanne system, which is the deposit
`full-text-acquisition-2026-08-10-repository-records.md` reached through
OpenAlex. A fifth index knowing of it, where three do not, is a second reading of
the same deposit rather than a new one, and the paragraph below says how that was
checked.

## A third shape of a door that reports as open

Two shapes are already recorded in this directory. A publisher endpoint returned
HTTP 200 and a document named `full-text-retrieval-response` carrying no article
text. An index named a repository copy that the repository does not serve. This
is a third, and it is the cheapest of the three to walk into.

Every CORE hit above carries a field named `fullText`, and its value on all three
is the sentence `Not available for public API users.` A route that read that
field as the article, or that treated it as present because it is a non-empty
string, would record three of the eight as obtained, and two of those three have
no deposit anywhere behind them. `downloadUrl` is empty on all three and
`sourceFulltextUrls` is empty on all three, which is where a file would be named
if one existed.

The output record behind the Lausanne hit is one level further in, and it does
name an address under that heading:

    curl -sL -m 60 "https://api.core.ac.uk/v3/search/works/?q=doi:%2210.1111/1556-4029.15431%22" \
      | python -c "import json,sys; print(json.load(sys.stdin)['results'][0]['outputs'])"
    ['https://api.core.ac.uk/v3/outputs/688722047']

    curl -sL -m 60 "https://api.core.ac.uk/v3/outputs/688722047" | python -c "
    import json,sys
    o=json.load(sys.stdin)
    print('downloadUrl=%r sourceFulltextUrls=%s' % (o.get('downloadUrl'), o.get('sourceFulltextUrls')))
    "
    downloadUrl='' sourceFulltextUrls=['https://iris.unil.ch/handle/iris/179025']

The address under `sourceFulltextUrls` is a landing page rather than a file, and
it resolves to the item already recorded:

    curl -sL -m 60 -o /dev/null -w "handle http=%{http_code} url=%{url_effective}\n" "https://iris.unil.ch/handle/iris/179025"
    handle http=200 url=https://iris.unil.ch/entities/publication/23b02376-f595-4008-bbbe-cd167fd6aaeb

That is the same entity identifier
`full-text-acquisition-2026-08-10-repository-records.md` recorded, whose only
candidate file is 3903 bytes and is refused with HTTP 401. So CORE and OpenAlex
reach one deposit and it is the deposit that holds no manuscript. Nothing here
adds a copy; what it adds is that a field named for full text, at an index, held
a landing page for a file that is refused.

## Two repositories asked directly

Every route in this directory so far has asked an index and believed its answer
about what a repository holds. The last record's finding was that an index can be
wrong about that, which is an argument for asking a repository itself. The
affiliations on the eight, read from OpenAlex, are mostly forensic institutes and
private practices with no public deposit interface. Two are universities running
a repository that answers a query, and those two were asked, for the studies
their people are named on:

    for base in https://tspace.library.utoronto.ca/server/api/discover/search/objects \
                https://ir.library.ontariotechu.ca/server/api/discover/search/objects; do
      for q in '"2D Ellipse Method"' '"bullet impacts"' 'Liscio' '"angle of impact"'; do
        curl -sL -m 60 "$base?query=<q, url encoded>&size=5"   # hit count and the first three titles
      done
    done

    TSpace (University of Toronto) | "2D Ellipse Method"    hits=0
    TSpace (University of Toronto) | "bullet impacts"       hits=2
    TSpace (University of Toronto) | Liscio                 hits=14
    TSpace (University of Toronto) | "angle of impact"      hits=31
    eScholarship (Ontario Tech)    | "2D Ellipse Method"    hits=0
    eScholarship (Ontario Tech)    | "bullet impacts"       hits=0
    eScholarship (Ontario Tech)    | Liscio                 hits=1
    eScholarship (Ontario Tech)    | "angle of impact"      hits=3

The hit counts are not near misses. The search matches on any term, so a query of
three words returns items sharing one of them, and the titles returned are about
brain injury, cold spray manufacturing and bloodstain patterns. No item
resembling any of the eight studies, and no thesis behind one, appears in either
repository.

This covers two institutions out of the affiliations on the eight and it is not a
sweep of the rest. The Netherlands Forensic Institute, which holds the one study
in `ellipse-accuracy.md` that compares methods across more than one material, was
not asked on this route, and neither were the remaining affiliations.

## No full text was obtained

For any of the eight. Nothing in `ellipse-accuracy.md` changes and its statement
that no material has a measured error tied to a stated angle range still holds,
so the material table still has no row it may lawfully carry and the published
cases still cannot be reproduced.

## What is still owed

The eight full texts.

Interlibrary loan, an institutional subscription and writing to the authors.
Those are the three routes issue #76 names that are not requests this kind of
route makes, and they are untried. That is unchanged and this file does not
soften it. `full-text-acquisition-2026-08-10.md` narrowed the second of them to
named credentials at two publishers and recorded that the third publisher exposes
no reader endpoint at all, so what that route needs is known even though it has
not been taken.

Asking the remaining affiliations directly, in the way the section above asks two
of them.

## What this file is not

It is not a claim that the eight cannot be obtained. Four indexes and now a
fifth not knowing of a deposit is not the deposit's absence, and the last record
is the standing example of an index being wrong in exactly that direction.

It is not a claim that reading a location list is a route that failed. It is
recorded here as a route that returns nothing at three indexes and something at a
fourth, which is a statement about those indexes rather than about the studies.
