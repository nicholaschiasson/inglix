# inglix <sub>/ ˈɪŋ glɪʃ /</sub>

English done right.

An opinionated, better system for spelling words in English.

## Preamble

<details>

<summary>Click to expand.</summary>

The English language is _wrought_ with inconsistent uses of letters in the alphabet for spelling out sounds. This is a source of confusion for just about everyone who has ever read and written any amount of English, learners and native speakers alike.

Imagine reading some text and coming across a word you haven't yet learned. In elementary schools in Canada, children are taught to "sound it out" in order to make an educated guess as to what the word might be based on some rules about generally how letters should sound. This works in a handful of cases, but becomes less useful for more complex words. There are exceptions to seemingly every rule, and for many words it almost seems as if the metaphorical _authors of the English language_, God rest their souls, were absolutely inebriated when coming up with these arbitrary spellings.

Now imagine writing an essay and believing that you have a vast vocabulary amongst your peers, when suddenly you try to write the word "written", as I had just done two paragraphs ago, and you need to stop to think about whether the word has one or two "t"s. "Sound it out..." I hear the faint memory of my grade one school teacher whisper, but frankly that doesn't help in this case. Does a double "t" _ever_ change the sound of the word in English? Does it actually matter? Okay, so google tells me it has two "t"s. Great, but then why does "writing" and "writer" only have one...? The difference in pronunciation, if any, is negligible (this could be my own accent at work though, your mileage may vary).

I could keep going though. What about other words like "gauge"? Why is it spelled that way? What part does that "u" actually play? I hear this word quite often actually at work and just about every non-native speaker pronounces it more like "gowge". This is intuition. I agree it should be pronounced that way given its spelling. But it's not. I don't usually correct people on this because I don't want to be annoying about something that doesn't matter that much. But English, you're putting me in a tough spot here!

What about all of these "O-U-G-H" sounds that are all _over_ the place? I mean seriously: "rough", "though", "through", "cough", "drought"! Enough is enough (:wink:)! That's five different pronunciations for the same four letter combination! This is only the tip of the iceberg though. English is riddled with weird spellings. The ones I've mentioned so far have been quite tame if I'm being honest.

Oh and maybe just as the cherry on top for that rant, even my own name, "Nicholas", is spelled ambiguously. I work with a lot of French speakers. The name also exists commonly among the French population, obviously, but they don't usually spell it with that "h" in there, so it's more often "Nicolas". And that's how people spell my name in DMs even though they can see my name written out right in front of them already _IN THE SOFTWARE_. If I'm being honest though, I don't usually go by "Nicholas". I usually go by "Nick"... :neutral_face: "_Nick_"... "_*NICK*_"! A "k"?! You're going to change the "h" to a "k" for the nickname?! So now, when people write to me, the French speakers spell my name "Nic", the English speakers spell my name "Nick", and the rest of the understandably confused population spells my name "Nich"! My own name is a mess! Of course, I'm exaggerating my frustration here. I don't really care at all. In fact, it's actually just kind of funny to me.

Of course, I understand that the origin story of the modern written English language is hairy and complex, and I understand that things will never really change. We need our systems to allow us to preserve our history. We need to keep as much information as accessible as possible. What that means is that we're stuck with the broken system, and probably for a long, long time. I have no real problem with that. It's maybe difficult to learn English, but in the end it is trivial. My complaints are real, but exaggerated.

In spite of all of that, I am starting this project as somewhat of a casual experiment to try to "fix" the English writing system. Somehow, this kind of thing is fun to me, and I think I actually have something quite nicely crafted so far.

The goal is simple: repurpose the letters in the English alphabet to make more sense. Simplify. Remove redundancy. Make it truly possible to "sound it out" with more confidence. I want to redefine the mappings from letters of the alphabet to possible sounds in the English language so that anybody can read or write with relative ease.

Of course it goes without saying that this will be quite opinionated. The dictionary will be entirely based on my own accent (Standard Canadian). It's entirely possible though that the system may be easily transposable to other accents. Hey, it might even just work right out of the box for Brits and I wouldn't know it. I guess time will tell.

So if you read through the first sentence of this preamble and thought to yourself, "Hmmm... what is this word, 'W-R-O-U-G-H-T'...? OHH... 'ROT'! I see. Didn't know that's how you spelled that word. :thinking:", or even if you read through this sentence accidentally pronouncing the word "read" as "reed" on your first pass instead of "red", then buckle up because this project is _MADE FOR YOU_! We're about to fix English and there is no turning back!

</details>

## Alphabet

### English

26 letters. 2 Versions of each...

```
A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
a b c d e f g h i j k l m n o p q r s t u v w x y z
```

## inglix

Only 33 letters total, carefully selected to represent all the sounds of English!

```
                I J         O       S T U V     Y
a b c d e f g h i j k l m n o p   r s t u v w x y z
```

## Graphemes

Graphemes are the written representations of the units of sound we can make when speaking, also known as phonemes. In other words, a grapheme is the letter(s) we use to represent a sound.

We will divide the list of English phonemes into two sets, [pure](#pure-graphemes) and [composite](#composite-graphemes), and assign inglix graphemes to those.

We will consider a pure phoneme to be an atomic sound in which the mouth shape remains relatively unchanged or steady for the duration of the sound.

Logically and as the name implies, we will consider composite phonemes to be sounds composed of more than one pure phoneme.

This distinction is useful since inglix aims to only address all pure phonemes, mapping a single letter to each possible pure phoneme. Conversely, the English writing system is inconsistent and commonly represents composite phonemes using single letters (case in point, the word "I"). The composite phonemes are listed to provide examples of how they are to be represented in inglix.

### Intuition Over Reason

<details>

<summary>Click to expand.</summary>

You will remark when reading the tables below that there exist some exceptional cases in inglix, some cases contradicting the rules set up above. For example the long "o" vowel sound equivalent, or the "ng" sound, to name a couple.

You may be wondering now if this doesn't go against the primary objective of the writing system. You are right to wonder that.

As it is the aim of inglix to strike a balance between covering as many pure phonemes as possible with single letters and remaining as straightforward and intuitive as possible to anybody with a knowledge of how English words can sound, cases such as these lean in favour of intuition, not _perfection_.

Basically, inglix opts for slight optimizations, omitting arguably less useful letters which would normally be used to denote implicit or nearly "silent" phonemes. In cases like these, it should be virtually impossible for ambiguity to arise for the majority of English readers (at least, among those who share the Standard Canadian accent).

See the [exceptions]() section for a comprehensive explanation for each case which breaks the "rules" of inglix.

</details>

### Pure Graphemes

#### Vowels

##### Short Vowels

| IPA Symbol | Common English Grapheme(s) | inglix Grapheme | English Example | inglix egzampl |
|-|-|-|-|-|
| æ | a | a | cat, bag, fan[^1] | kat, bag, fan |
| e | e | e | leg | leg |
| ɪ | i | i | sit | sit |
| ɒ | o | o | top | top |
| ʊ | oo, u, o, ou | u | book, put, wolf, would | buk, put, wulf, wud |
| ʌ | u | V | rub | rVb |

[^1]: Wondering why you pronounce these three words differently even though they are supposedly the same sound? See the section about the [short "a" vowel sound exception](#short-a-vowel-and-others) for details about that. In summary, let your accent shine. Consider this your final taste of freedom of expression before inglix takes it all away...

##### Long Vowels

It is commonly considered that there are 5 long vowels but the "ie"/"igh" sound is very clearly a composition of two different vowels. Thus it is omitted from the list of [pure](#pure-graphemes) phonemes and instead listed as a [composite](#composite-graphemes) phoneme.

| IPA Symbol | Common English Grapheme(s) | inglix Grapheme | English Example | inglix egzampl |
|-|-|-|-|-|
| i | ee, ea | I | bee, beat | bI, bIt |
| ou | oe, ow | O[^2] | toe, flow | tO, flO |
| u | oo, ue | U | moon, cue | mUn, kIU |

[^2]: This is actually a composite phoneme in disguise. To understand why it is listed here, see the section about the [long "o" vowel sound exception](#long-o-vowel).

##### R-Controlled Vowels

These are special case vowel sounds that are only made in English when followed immediately by the letter "r".

As with long vowels, there are commonly considered more r-controlled vowels than inglix will bother with. In inglix "ar" and "or" sounds are trivial and can be represented using composition (see the [composite graphemes](#composite-graphemes) below).

| IPA Symbol | Common English Grapheme(s) | inglix Grapheme | English Example | inglix egzampl |
|-|-|-|-|-|
| ɜr | er, ir, ur | yr | her, bird, hurt | hyr, byrd, hyrt |

#### Consonants

| IPA Symbol | Common English Grapheme(s) | inglix Grapheme | English Example | inglix egzampl |
|-|-|-|-|-|
| b | b | b | bet | bet |
| tʃ | ch | c | chain | ceIn |
| d | d | d | dip | dip |
| f | f | f | fan | fan |
| ɡ | g | g | gap | gap |
| ʒ | ge, z, si, j | J | beige, azure, vision | beIJ, aJyr, viJin |
| h | h | h | hat | hat |
| dʒ | j | j | Jane | jeIn |
| k | k, c | k | cap | kap |
| l | l | l | leg | leg
| m | m | m | met | met |
| n | n | n | net | net |
| p | p | p | pet | pet |
| r | r | r | rat | rat |
| s | s | s | sue | sU |
| ʃ | sh | x | ship | xip |
| t | t | t | tip | tip |
| θ | th | S[^3] | thin | Sin |
| ð | th | T | this | Tis |
| v | v | v | van | van |
| w | w | w[^4] | wet | wet |
| j | y | Y[^5] | yak | Yak |
| z | z | z | zoo | zU |

[^3]: This may seem to be a bizarre choice of grapheme for the unvoiced "th" sound. The fact is that it was chosen because a stereotypical "s" lisp tends to make this sound also. So why not use capital "S" to represent it? It's almost too perfect!

[^4]: There is a case to be made against "w" being included in inglix. Instead, it could have probably simply been replaced by the inglix "U". This would be similar to how the French language uses "ou" in place of the "w" sound (for native French words). Given that, "w" is somewhat elegant, instantly recognizable, and intuitive in its own right. Using a "U" in its place might confuse readers into emphasizing the "U" sound, making some words sound awkward. Lowercase "w" can stay.

[^5]: Like "w", the same goes for the "y" sound. This could have probably been replaced by the inglix "I" but it would likely serve more to confuse readers and also force over-emphasis in some places. Since "y" is already reserved for the "er" sound via "yr", the uppercase version will have to suffice.

### Composite Graphemes

These are primarily for example purposes. Some of the composites listed below are self-evident and only listed due to the fact that they are commonly treated as unique phonemes but in practice sound indistinguishable from their parts. However, there are a few items in the list below which are perhaps good to note, such as the "ie"/"igh" long vowel sound and the "ng" sound.

| IPA Symbol | Common English Grapheme(s) | inglix Grapheme | English Example | inglix egzampl |
|-|-|-|-|-|
| aɪ | ie, igh | aI | pie, high | paI, haI |
| eɪ | ai, ay | eI | paid, tray | peId, creI |
| ɑr | ar | or[^6] | car | kor |
| ɔr | or | Or[^7] | cork | kOrk |
| au | ow, ou | aU | cow, out | kaU, aUt |
| ɔɪ | oy, oi | OI | boy, void | bOI, vOId |
| ɪər | eer, ear | Ir | deer, near | dIr nIr |
| er | air, ere | er | hair, there | her, Ter |
| ur | our | Ur | tour | tUr |
| ŋɡ | ng | ng[^8] | sing, tongue | sing, tung |
| ŋk | nk | nk | pink | pink |
| ks | x | ks | box | boks |
| kw | qu | kw | quit | kwit |

[^6]: This specific composite phoneme is actually a tricky case and still up for debate for its representation in inglix. Examples "car" and "start" would be spelled "kor" and "stort" in the current solution... But this might confuse many. The "o" graphene seems to more closely fit the sound (it's true, say "car" slowly and then say "top" slowly and see how similar they are in sound), but in reality, it is technically a different phoneme. This one may change. Perhaps it will be placed up in the [R-Controlled Vowels](#r-controlled-vowels) section (where it actually belongs!).

[^7]: Yet another exceptional case. While this one might be somewhat less noticeable, it is in fact a misrepresentation of the true sounds you make when vocalizing this sound. See the section on the [r-controlled long "o" vowel exception](#r-controlled-long-o-vowel) for an explanation.

[^8]: If you noticed that the "n" in an "ng" or "nk" sound is not the same tip-of-the-tongue regular "n" sound, then you are indeed a keen one. Otherwise, you may be interested to learn just why this is an exception. See the section about the ["ng" vowel exception](#ngnk-vowel) for some explanation.

### Exceptions

#### Short "a" Vowel (And Others?)

Depending on a speaker's accent, the words "cat", "bag", and "fan", as well as many other words with the supposed short "a" vowel sound, may all have slightly different sounds. The inglix system will not attempt to address these differences, allowing them to exist to support expression of different accents.

You may find other peculiarities like this, perhaps for the other vowel sounds, but according to the Standard Canadian accent, this is pretty much the only one of note.

#### Long "o" Vowel

Technically, this is a composite phoneme, but inglix will consider it pure for efficiency's sake. When vocalizing this sound, during the first part, the mouth shape starts in a unique position (which will not be explicitly identified by an inglix grapheme) before transitioning to the shape the mouth makes for the long "u" vowel sound. Try it yourself and see if you can notice how the mouth changes when pronouncing this sound. It is this first position that makes things tricky, because it is (probably) only ever made for this specific combination sound, that is to say before shifting directly to the long "u" vowel sound. It is as if English was built to make it uncomfortable or rare for that sound to ever exist without being followed by the long "u" vowel sound.

One way to address this would be to use a graphene for that part and then combine it with the long "u" vowel graphene. We could do something like "OU". However, since, as was mentioned, this "O" only ever comes before "U" anyway, maybe it's preferable to make an exception. Toss the "U" and use the "O" to represent this specific composite phoneme, even though in general that goes against the rules which were established above: one letter maps to one pure phoneme.

#### R-Controlled Long "o" Vowel

This one is less obvious, but try saying the words "hope" and "port" slowly and see if you can tell that the "o" sound in there is actually slightly different. In "port", the vowel never transitions to that long "u" sound. In fact, a different IPA symbol is even used for the first, more long "o" sound as well, but to my (Standard Canadian accent) ear, they sound equivalent. As such, this r-controlled long "o" vowel sound does not merit its own unique grapheme. Instead it can share with "O", and pretty much everybody should understand.

#### "ng"/"nk" Vowel

It can be said that the "ng" as well as "nk" sounds make a completely different mouth shape compared to the solitary "n" sound, and thus inglix should not use the letter "n" in this composite grapheme. Rather than the typical "n" sound, which has the front of the tongue touch the top of the mouth, it is the back of the tongue used in these "ng" and "nk" sounds.

In English, it is incredibly rare to have that back of the tongue "n" sound appear without being followed by a "g" or a "k" sound. This is something that comes naturally to English speakers. As such, for this case, rather than associating a new letter to the sound for the inglix system, we will trim the fat and leave it up to intuition.

## Homonyms

To express homonyms, inglix chooses a unique approach. This is where the trivia starts to emerge.

The inglix dictionary will identify homonyms uniquely by duplicating the first letter of the word for each instance of a homonym.

Take the words "to", "too", and "two", for example.

In inglix, we might write these as "tU", "ttU", and "tttU", respectively. It will be up to you to remember "wic wwic iz wic" (which witch is which). However since it is uncommon for there to be more than 3 homonyms for a given sounding word, this approach seems quite clean and elegant being the only case of double letters in all of inglix.

## Syntax

The inglix writing system is made for writing English. Therefore, it works basically the same way! However, there are some key differences to take note of!

### Casing

In inglix, you do not capitalize the first letter of a sentence of even those of proper nouns! Every version of a letter, that is to say uppercase and lowercase, is reserved intentionally to express words phonetically. This allows for inglix to be much, much simpler than English, and even strip down the alphabet a good deal.

Perhaps the best way to get started reading and writing inglix is to not think of letters in terms of uppercase and lowercase anymore at all, but rather as a set of unique letters each with a resulting sound (some of which happen to only be accessible via another letter on the keyboard in tandem with the shift key :sweat_smile:)

### Proper Nouns

It was just mentioned how proper nouns need not be capitalized in inglix, but in truth, it may serve to confuse people even spelling proper nouns in inglix at all.

It is up to the writer to decide how they wish to spell proper nouns. They may choose to lean into inglix and spell proper nouns in good inglix, but it is also perfectly acceptable in an inglix text to spell proper nouns in English, even with the capitalized first letter. This is not forbidden.

### Punctuation

Punctuation is to be used in the same way as it is already used in English.

### Apostrophes

Apostrophes are notable. In inglix, they will be used in the same inglix words as their English counterparts.

For example, the words "there", "their", and "they're" you might think to spell as "Ter", "TTer", and "TTTer" in inglix. Instead, we can promote comprehension by continuing to use contractions the same way we already do in English. Therefore, "they're" would be spelled "Te'r", reducing the number of our identically spelled words and adding a bit of extra _flavour_ to the inglix.

### Hyphenated words

Hyphens can be used in inglix to contract words in the exact same way that they would be used in English, but note that this does not impact identically spelled words in the same way that apostrophes sort of seem to.

For example, consider the words "marry" and "merry". We might spell them respectively as "merI" and "mmerI". When we go to spell the word "merry-go-round", we still have to spell "merry" the way it is spelled on its own. So this becomes "mmerI-go-raUnd".

Perhaps it seems obvious, but it is important to note.

## To Do

- Transpiler script
- Dictionary
- Alphabet reference
- Write inglix version of this readme file
