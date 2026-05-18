# workflow.packet_archive_reader

`workflow.packet_archive_reader` rehydrates a saved packet wave archive from disk.

It is intentionally strict:

- `wave_label` must match the packet writer contract
- manifest paths must be absolute
- manifest paths must resolve inside the targeted archive directory
- worker/shared packet JSON files must deserialize cleanly
- worker/shared packet target kinds must match their expected roles

The read report returns both the raw archive-facing documents and a reconstructed
`FeatureWavePacketBundle` so later consumers can work from typed packet data.

One current archive-format limitation remains: worker `selection_reason` is not
persisted by `packet_archive_writer`, so the rehydrated worker bundles preserve
the stored packet contract and lane assignment data while leaving
`selection_reason` blank.
