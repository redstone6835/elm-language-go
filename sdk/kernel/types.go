package kernel

// Status is the fixed signed status value returned across the carrier ABI.
type Status int32

const (
	// StatusOK reports successful completion.
	StatusOK Status = 0
)

// Owner identifies one ELM generation. It is metadata for lifecycle and
// resource ownership, not an authority token supplied on every direct call.
type Owner struct {
	CellID     uint64
	Generation uint64
}

// Valid reports whether both owner identity components are non-zero.
func (owner Owner) Valid() bool {
	return owner.CellID != 0 && owner.Generation != 0
}

// Handle is a fixed-layout opaque kernel resource identity. Generated SDK
// packages wrap it in a resource-specific type before exposing operations.
type Handle struct {
	kind       uint32
	flags      uint32
	slot       uint64
	generation uint64
}

// Valid reports whether the opaque identity can refer to a live resource.
func (handle Handle) Valid() bool {
	return handle.kind != 0 && handle.slot != 0 && handle.generation != 0
}
