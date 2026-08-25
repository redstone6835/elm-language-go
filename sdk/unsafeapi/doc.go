// Package unsafeapi contains resource-specific opaque values for generated
// MMIO, DMA, IRQ, and raw-address bindings.
//
// The name is intentional: admission checks and typed handles do not make
// arbitrary kernel memory access safe. Operations are generated only for EKI
// symbols and capabilities admitted for the package.
package unsafeapi
