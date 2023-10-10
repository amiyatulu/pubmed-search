
```
curl "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi?db=nuccore" -d 'term="Hepatitis C"[Organism] AND 7000:10000[SLEN]'

curl "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi?db=pubmed&retmax=50" -d 'term=Hepatitis C'

curl "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=pubmed&format=xml" -d 'id=24102982'


```

```

from Bio import Entrez

Entrez.email = "Your.Name.Here@example.org"

handle = Entrez.efetch(db="nucleotide", id="AY851612", rettype="gb", retmode="text")

print(handle.readline().strip())
LOCUS       AY851612                 892 bp    DNA     linear   PLN 10-APR-2007

handle.close()


```

