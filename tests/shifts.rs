use sangoku_test_support::{
    Result,
    shifts::{self, OPS, Spec},
};
#[test]
fn constant_candidates_match_host_and_preserve_frames() -> Result<()> {
    for width in [16, 32] {
        for &op in OPS {
            for n in [8, (width - 1) as u8] {
                let s = Spec {
                    op: op.into(),
                    width,
                    count: Some(n),
                };
                for variant in ["lanes", "sdcc-speed"] {
                    let b = shifts::build(&s, variant, None, None)?;
                    for value in [1u32 << (width - 1), 0x12345678 & s.mask()] {
                        for mode in ["body", "call-live"] {
                            let r = shifts::run_case(&b, &s, value, n, mode, 0xD000, 0xA5)?;
                            assert!(r.correct, "{s:?} {variant} {mode} value={value:X}: {r:?}");
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
#[test]
fn variable_candidates_mask_counts_and_preserve_frames() -> Result<()> {
    for width in [16, 32] {
        for &op in OPS {
            let s = Spec {
                op: op.into(),
                width,
                count: None,
            };
            for variant in ["loop", "byte-loop"] {
                let b = shifts::build(&s, variant, None, None)?;
                for count in [0, 8, (width + 1) as u8, 255] {
                    for value in [1u32 << (width - 1), 0x12345678 & s.mask()] {
                        for mode in ["body", "call-live"] {
                            let r = shifts::run_case(&b, &s, value, count, mode, 0xD000, 0x55)?;
                            assert!(
                                r.correct,
                                "{s:?} {variant} {mode} n={count} value={value:X}: {r:?}"
                            );
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
#[test]
fn pair_left_shift_and_state_independence() -> Result<()> {
    for count in [1, 8] {
        let s = Spec {
            op: "shl".into(),
            width: 16,
            count: Some(count),
        };
        let b = shifts::build(&s, "pair", None, None)?;
        for stack in [0xC900, 0xD002] {
            for poison in [0xA5] {
                for value in [0xFEDC] {
                    for mode in ["body", "call-live"] {
                        let r = shifts::run_case(&b, &s, value, count, mode, stack, poison)?;
                        assert!(r.correct, "{s:?} {mode} {r:?}");
                    }
                }
            }
        }
    }
    Ok(())
}
#[test]
fn variable_state_independence() -> Result<()> {
    for width in [16, 32] {
        for &op in OPS {
            let s = Spec {
                op: op.into(),
                width,
                count: None,
            };
            for variant in ["loop", "byte-loop"] {
                let b = shifts::build(&s, variant, None, None)?;
                for stack in [0xC900, 0xD002] {
                    for poison in [0xA5] {
                        for count in [0, 255] {
                            let r = shifts::run_case(
                                &b,
                                &s,
                                0x8765FEDC & s.mask(),
                                count,
                                "body",
                                stack,
                                poison,
                            )?;
                            assert!(r.correct, "{s:?} {variant} n={count}: {r:?}");
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
#[test]
fn inline_removes_call_and_return_overhead() -> Result<()> {
    for width in [16, 32] {
        let s = Spec {
            op: "shr".into(),
            width,
            count: Some(8),
        };
        let b = shifts::build(&s, "bytes", None, None)?;
        let a = shifts::run_case(&b, &s, 0xDEADBEEF, 8, "call", 0xD000, 0x55)?;
        let c = shifts::run_case(&b, &s, 0xDEADBEEF, 8, "inline", 0xD000, 0x55)?;
        assert!(a.correct && c.correct);
        assert!(a.cycles >= c.cycles + 40);
        assert_eq!(c.peak, 0);
    }
    Ok(())
}
#[test]
fn checked_rust_counts_differ_from_masked_clif_counts() {
    for n in [16, 17, 255, 65536, u32::MAX] {
        assert!(1u16.checked_shl(n).is_none());
        assert_eq!(
            shifts::expected(
                &Spec {
                    op: "shl".into(),
                    width: 16,
                    count: None
                },
                1,
                n as u8
            ),
            1u16.wrapping_shl(n) as u32
        );
    }
    for n in [32, 33, 255, 65536, u32::MAX] {
        assert!(1u32.checked_shr(n).is_none());
        assert_eq!(
            shifts::expected(
                &Spec {
                    op: "shr".into(),
                    width: 32,
                    count: None
                },
                1,
                n as u8
            ),
            1u32.wrapping_shr(n)
        );
    }
}
