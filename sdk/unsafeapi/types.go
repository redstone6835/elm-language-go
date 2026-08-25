package unsafeapi

import "github.com/redstone6835/elm-language-go/sdk/kernel"

// MMIORegion is an admitted device register window.
type MMIORegion struct {
	handle kernel.Handle
	length uint64
}

// Valid reports whether the region has a live handle and non-empty range.
func (region MMIORegion) Valid() bool {
	return region.handle.Valid() && region.length != 0
}

// Length reports the admitted byte length.
func (region MMIORegion) Length() uint64 {
	return region.length
}

// DMABuffer is an owner-bound DMA allocation. DeviceAddress is meaningful to
// the device or IOMMU domain selected at admission time.
type DMABuffer struct {
	handle        kernel.Handle
	deviceAddress uint64
	length        uint64
}

// Valid reports whether the allocation has a live handle and non-empty range.
func (buffer DMABuffer) Valid() bool {
	return buffer.handle.Valid() && buffer.length != 0
}

// DeviceAddress reports the address visible to the admitted device domain.
func (buffer DMABuffer) DeviceAddress() uint64 {
	return buffer.deviceAddress
}

// Length reports the allocation length in bytes.
func (buffer DMABuffer) Length() uint64 {
	return buffer.length
}

// IRQ is an admitted interrupt source.
type IRQ struct {
	handle   kernel.Handle
	sourceID uint64
}

// Valid reports whether the interrupt identity is usable.
func (irq IRQ) Valid() bool {
	return irq.handle.Valid() && irq.sourceID != 0
}

// SourceID reports the admitted interrupt source identifier.
func (irq IRQ) SourceID() uint64 {
	return irq.sourceID
}
