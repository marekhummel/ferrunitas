//! Selected physical constants.
//!
//! Exposes common constants (e.g. gravitational acceleration) as strongly
//! typed measures for convenient reuse.

// Note that we cant fully leverage the operators for units due to the const.
// Hence the manual entry of all values

use crate::model::measure::Measure;
use crate::system::*;
use crate::unit;
use typenum::*;

// Constant related units
unit!(compound: CoulombPerMole, "C/mol", [(Coulomb, P1), (Mole, N1)]);
unit!(compound: NewtonPerAmpereSquared, "N/A²", [(Newton, P1), (Ampere, N2)]);
unit!(compound: OnePerMole, "mol⁻¹", [(One, P1), (Mole, N1)]);
unit!(compound: JoulePerMoleKelvin, "J/(mol·K)", [(Joule, P1), (Mole, N1), (Kelvin, N1)]);
unit!(compound: HertzPerVolt, "Hz/V", [(Hertz, P1), (Volt, N1)]);
unit!(compound: CubicMetrePerKilogramSecondSquared, "m³/(kg·s²)", [(CubicMetre, P1), (Kilogram, N1), (Second, N2)]);
unit!(compound: WattPerSquareMetreKelvinToTheFourth, "W/(m²·K⁴)", [(Watt, P1), (Metre, N2), (Kelvin, N4)]);
unit!(compound: MetreKelvin, "m·K", [(Metre, P1), (Kelvin, P1)]);
unit!(compound: PerGigaElectronVoltSquared, "1/(GeV²)", [(One, P1), (GigaElectronVolt, N2)]);
unit!(compound: KilometrePerSecondPerMegaparsec, "km/(s·Mpc)", [(Kilometre, P1), (Second, N1), (Megaparsec, N2)]);

// ==============================================================================

// Natural and Physical Constants in Rust

// =============================================================================
// SI DEFINING CONSTANTS (exact values by definition as of 2019 revision)
// =============================================================================

// Speed of light in vacuum (c)
pub const SPEED_OF_LIGHT: Measure<MetrePerSecond> = Measure::new_const(299_792_458.0);

// Planck constant (h)
pub const PLANCK_CONSTANT: Measure<JouleSecond> = Measure::new_const(6.626_070_15e-34);

// Elementary charge (e)
pub const ELEMENTARY_CHARGE: Measure<Coulomb> = Measure::new_const(1.602_176_634e-19);

// Boltzmann constant (k_B)
pub const BOLTZMANN_CONSTANT: Measure<JoulePerKelvin> = Measure::new_const(1.380_649e-23);

// Avogadro constant (N_A)
pub const AVOGADRO_CONSTANT: Measure<OnePerMole> = Measure::new_const(6.022_140_76e23);

// Luminous efficacy of 540 THz radiation (K_cd)
pub const LUMINOUS_EFFICACY_540_THZ: Measure<LumenPerWatt> = Measure::new_const(683.0);

// Hyperfine transition frequency of caesium-133 (Δν_Cs)
pub const CAESIUM_133_FREQUENCY: Measure<Hertz> = Measure::new_const(9_192_631_770.0);

// =============================================================================
// DERIVED CONSTANTS FROM SI DEFINING CONSTANTS
// =============================================================================

// Reduced Planck constant (ℏ = h/(2π))
pub const REDUCED_PLANCK_CONSTANT: Measure<JouleSecond> = Measure::new_const(1.054_571_817e-34);

// Electric constant (vacuum permittivity) ε₀
pub const ELECTRIC_CONSTANT: Measure<FaradPerMetre> = Measure::new_const(8.854_187_812_8e-12);

// Magnetic constant (vacuum permeability) μ₀ = 4π × 10⁻⁷ exactly
pub const MAGNETIC_CONSTANT: Measure<HenryPerMetre> = Measure::new_const(1.256_637_062e-6);

// Impedance of free space Z₀ = √(μ₀/ε₀)
pub const IMPEDANCE_OF_FREE_SPACE: Measure<Ohm> = Measure::new_const(376.730_313_668);

// Conductance quantum G₀ = 2e²/h
pub const CONDUCTANCE_QUANTUM: Measure<Siemens> = Measure::new_const(7.748_091_729e-5);

// von Klitzing constant R_K = h/e²
pub const VON_KLITZING_CONSTANT: Measure<Ohm> = Measure::new_const(25_812.807_45);

// Magnetic flux quantum Φ₀ = h/(2e)
pub const MAGNETIC_FLUX_QUANTUM: Measure<Weber> = Measure::new_const(2.067_833_848e-15);

// Josephson constant K_J = 2e/h
pub const JOSEPHSON_CONSTANT: Measure<HertzPerVolt> = Measure::new_const(4.835_978_484e14);

// =============================================================================
// FUNDAMENTAL PHYSICAL CONSTANTS
// =============================================================================

// Gravitational constant G
pub const GRAVITATIONAL_CONSTANT: Measure<CubicMetrePerKilogramSecondSquared> =
    Measure::new_const(6.674_30e-11);

// Fine structure constant α = e²/(4πε₀ℏc)
pub const FINE_STRUCTURE_CONSTANT: Measure<One> = Measure::new_const(7.297_352_566_4e-3);

// Inverse fine structure constant 1/α
pub const INVERSE_FINE_STRUCTURE_CONSTANT: Measure<One> = Measure::new_const(137.035_999_084);

// Stefan-Boltzmann constant σ = 2π⁵k⁴/(15h³c²)
pub const STEFAN_BOLTZMANN_CONSTANT: Measure<WattPerSquareMetreKelvinToTheFourth> =
    Measure::new_const(5.670_374_419e-8);

// Wien displacement constant b
pub const WIEN_DISPLACEMENT_CONSTANT: Measure<MetreKelvin> = Measure::new_const(2.897_771_955e-3);

// Rydberg constant R∞
pub const RYDBERG_CONSTANT: Measure<ReciprocalMetre> = Measure::new_const(10_973_731.568_160);

// =============================================================================
// PARTICLE PHYSICS CONSTANTS
// =============================================================================

// Electron rest mass
pub const ELECTRON_MASS: Measure<Kilogram> = Measure::new_const(9.109_383_701_5e-31);

// Proton rest mass
pub const PROTON_MASS: Measure<Kilogram> = Measure::new_const(1.672_621_923_69e-27);

// Neutron rest mass
pub const NEUTRON_MASS: Measure<Kilogram> = Measure::new_const(1.674_927_498_04e-27);

// Classical electron radius r_e = e²/(4πε₀m_e c²)
pub const CLASSICAL_ELECTRON_RADIUS: Measure<Metre> = Measure::new_const(2.817_940_326_2e-15);

// Electron Compton wavelength λ_C = h/(m_e c)
pub const ELECTRON_COMPTON_WAVELENGTH: Measure<Metre> = Measure::new_const(2.426_310_217_5e-12);

// Bohr radius a₀ = 4πε₀ℏ²/(m_e e²)
pub const BOHR_RADIUS: Measure<Metre> = Measure::new_const(5.291_772_109_03e-11);

// Bohr magneton μ_B = eℏ/(2m_e)
pub const BOHR_MAGNETON: Measure<JoulePerTesla> = Measure::new_const(9.274_010_078_3e-24);

// Nuclear magneton μ_N = eℏ/(2m_p)
pub const NUCLEAR_MAGNETON: Measure<JoulePerTesla> = Measure::new_const(5.050_783_646_1e-27);

// =============================================================================
// ATOMIC AND NUCLEAR CONSTANTS
// =============================================================================

// Atomic mass constant (unified atomic mass unit)
pub const ATOMIC_MASS_CONSTANT: Measure<Kilogram> = Measure::new_const(1.660_539_066_60e-27);

// Electron g-factor g_e
pub const ELECTRON_G_FACTOR: Measure<One> = Measure::new_const(-2.002_319_304_362_56);

// Proton g-factor g_p
pub const PROTON_G_FACTOR: Measure<One> = Measure::new_const(5.585_694_689_3);

// Neutron g-factor g_n
pub const NEUTRON_G_FACTOR: Measure<One> = Measure::new_const(-3.826_085_45);

// Fermi coupling constant G_F/(ℏc)³
pub const FERMI_COUPLING_CONSTANT: Measure<PerGigaElectronVoltSquared> =
    Measure::new_const(1.166_378_7e-5);

// =============================================================================
// CONVERSION CONSTANTS
// =============================================================================

// Hartree energy E_h = 2R∞hc
pub const HARTREE_ENERGY: Measure<Joule> = Measure::new_const(4.359_744_722_207_1e-18);

// Standard acceleration of gravity g₀
pub const STANDARD_GRAVITY: Measure<MetrePerSecondSquared> = Measure::new_const(9.806_65);

// Gas constant R = N_A × k_B
pub const GAS_CONSTANT: Measure<JoulePerMoleKelvin> = Measure::new_const(8.314_462_618);

// Faraday constant F = N_A × e
pub const FARADAY_CONSTANT: Measure<CoulombPerMole> = Measure::new_const(96_485.332_12);

// =============================================================================
// ASTROPHYSICAL AND COSMOLOGICAL CONSTANTS
// =============================================================================

// Hubble constant H₀ (approximate, varies by measurement)
pub const HUBBLE_CONSTANT: Measure<KilometrePerSecondPerMegaparsec> = Measure::new_const(70.0);
