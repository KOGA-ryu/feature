# Speed Reading Mechanics Notes

This note records the reading-mechanics assumptions behind
`scroll_reader_capsule_v1`.

## Locked-In Model

The focus word is an eye anchor, not the reading unit.

The reading unit is the visible context band around the focus anchor. The user
should be able to read words ahead of and behind the anchor through peripheral
vision. The anchor helps the eye sit in a useful position while the surrounding
sentence context carries meaning.

Implementation consequences:

- keep exactly one centered focus anchor word
- keep at least several readable context words ahead and behind when available
- bias the context budget slightly ahead of the focus anchor for left-to-right
  English, while preserving enough behind-context for comprehension
- keep context opacity high enough to read, not merely decorate
- fade only toward capsule edges
- preserve wheel rewind because regressions/re-reading are part of normal
  comprehension support
- do not turn the pill into one-word RSVP

## Research Notes

- Rayner, Schotter, Masson, Potter, and Treiman describe normal skilled reading
  as involving language processing, eye movements, and comprehension of words,
  phrases, and sentences rather than isolated word recognition. They also note
  the normal silent reading range of roughly 200-400 wpm and warn that speed and
  comprehension trade off when the process becomes more like skimming.
  Source: https://journals.sagepub.com/doi/10.1177/1529100615623267

- Skilled readers make regressions a meaningful fraction of the time. That
  supports keeping manual rewind and not treating backward motion as an error.
  Source: https://journals.sagepub.com/doi/10.1177/1529100615623267

- RSVP-style presentation is a known speed-reading technology pattern, but the
  review treats it separately from normal reading because it removes normal line
  navigation and preview conditions. The capsule should avoid becoming RSVP.
  Source: https://journals.sagepub.com/doi/10.1177/1529100615623267

- Parafoveal preview gives readers information before direct fixation and can
  shorten later fixation time. This supports making surrounding words readable,
  especially words ahead of the focus anchor.
  Source: https://tmalsburg.github.io/SchotterEtAl2018.pdf

- Skilled reading uses fixations and saccades, not a smooth visual scan. In the
  parafoveal-processing review, typical fixations are described as roughly
  200-250 ms, direct fixation happens on about 70% of words, about 30% of words
  are skipped, and about 10-15% of saccades regress to previous text. This
  supports a fixed focus anchor, visible context, skipped-word tolerance, and
  manual rewind.
  Source: https://www.researchgate.net/publication/51759936_Parafoveal_processing_in_reading

- The perceptual span for English is asymmetric: useful information extends
  farther to the right of fixation than to the left. This supports making V1's
  context budget slightly ahead-weighted while still keeping behind-context
  readable.
  Source: https://www.tandfonline.com/doi/full/10.1080/13506285.2013.879084

## V1 Defaults From These Notes

- `context_text_opacity_percent = 72`
- `focus_text_opacity_percent = 100`
- `min_readable_context_words_each_side = 3`
- `ahead_context_budget_percent = 60`
- `behind_context_budget_percent = 40`
- focus anchor is centered at the fixed lens point
- context band stays readable and continuous
