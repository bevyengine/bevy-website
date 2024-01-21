//! This crate is used to validate the rust code of the `bevy` website.
//!
//! It is currently used to validate the rust code of the offical `bevy` book.
//! The modules represents the folder structure of the website.

mod learn {
    #[doc = include_str!("../../content/learn/quick-start/_index.md")]
    mod quick_start {
        #[doc = include_str!("../../content/learn/quick-start/next-steps/_index.md")]
        mod next_steps {}
    }
    #[doc = include_str!("../../content/learn/book/_index.md")]
    mod book {
        #[doc = include_str!("../../content/learn/book/assets/_index.md")]
        mod assets {
            #[doc = include_str!("../../content/learn/book/assets/custom-assets/_index.md")]
            mod custom_assets {}
            #[doc = include_str!("../../content/learn/book/assets/hot-reloading/_index.md")]
            mod hot_reloading {}
            #[doc = include_str!("../../content/learn/book/assets/loading-assets/_index.md")]
            mod loading_assets {}
            #[doc = include_str!("../../content/learn/book/assets/scenes-reflection/_index.md")]
            mod scenes_reflection {}
            #[doc = include_str!("../../content/learn/book/assets/working-with-handles/_index.md")]
            mod working_with_handles {}
        }

        #[doc = include_str!("../../content/learn/book/audio/_index.md")]
        mod audio {
            #[doc = include_str!("../../content/learn/book/audio/audio-basics/_index.md")]
            mod audio_basics {}
        }

        #[doc = include_str!("../../content/learn/book/development-practices/_index.md")]
        mod development_practices {
            #[doc = include_str!("../../content/learn/book/development-practices/boilerplate-reduction/_index.md")]
            mod boilerplate_reduction {}
            #[doc = include_str!("../../content/learn/book/development-practices/error-handling/_index.md")]
            mod error_handling {}
            #[doc = include_str!("../../content/learn/book/development-practices/fast-compiles/_index.md")]
            mod fast_compiles {}
            #[doc = include_str!("../../content/learn/book/development-practices/organizing-your-code/_index.md")]
            mod organizing_your_code {}
            #[doc = include_str!("../../content/learn/book/development-practices/testing/_index.md")]
            mod testing {}
        }

        #[doc = include_str!("../../content/learn/book/ecs/_index.md")]
        mod ecs {
            #[doc = include_str!("../../content/learn/book/ecs/commands/_index.md")]
            mod commands {}
            #[doc = include_str!("../../content/learn/book/ecs/change-detection/_index.md")]
            mod change_detection {}
            #[doc = include_str!("../../content/learn/book/ecs/entities-components/_index.md")]
            mod entities_components {}
            #[doc = include_str!("../../content/learn/book/ecs/exclusive-world-access/_index.md")]
            mod exclusive_world_access {}
            #[doc = include_str!("../../content/learn/book/ecs/systems-queries/_index.md")]
            mod systems_queries {}
            #[doc = include_str!("../../content/learn/book/ecs/resources/_index.md")]
            mod resources {}
        }

        #[doc = include_str!("../../content/learn/book/game-logic/_index.md")]
        mod game_logic {
            #[doc = include_str!("../../content/learn/book/game-logic/async-tasks/_index.md")]
            mod async_tasks {}
            #[doc = include_str!("../../content/learn/book/game-logic/custom-runners-headless-operations/_index.md")]
            mod custom_runners_headless_operation {}
            #[doc = include_str!("../../content/learn/book/game-logic/events/_index.md")]
            mod events {}
            #[doc = include_str!("../../content/learn/book/game-logic/run-criteria/_index.md")]
            mod run_criteria {}
            #[doc = include_str!("../../content/learn/book/game-logic/states/_index.md")]
            mod states {}
            #[doc = include_str!("../../content/learn/book/game-logic/system-ordering/_index.md")]
            mod system_ordering {}
            #[doc = include_str!("../../content/learn/book/game-logic/time-timers/_index.md")]
            mod time_timers {}
        }

        #[doc = include_str!("../../content/learn/book/graphics/_index.md")]
        mod graphics {
            #[doc = include_str!("../../content/learn/book/graphics/2d/_index.md")]
            mod two_dimensional {
                #[doc = include_str!("../../content/learn/book/graphics/2d/sprite-sheets/_index.md")]
                mod sprite_sheets {}
                #[doc = include_str!("../../content/learn/book/graphics/2d/sprites/_index.md")]
                mod sprites {}
            }
            #[doc = include_str!("../../content/learn/book/graphics/3d/_index.md")]
            mod three_dimensional {
                #[doc = include_str!("../../content/learn/book/graphics/3d/meshes/_index.md")]
                mod meshes {}
                #[doc = include_str!("../../content/learn/book/graphics/3d/pbr/_index.md")]
                mod pbr {}
            }
            #[doc = include_str!("../../content/learn/book/graphics/cameras/_index.md")]
            mod cameras {}
            #[doc = include_str!("../../content/learn/book/graphics/parent-child-hierarchy/_index.md")]
            mod parent_child_hierarchy {}
            #[doc = include_str!("../../content/learn/book/graphics/rendering-internals/_index.md")]
            mod rendering_internals {
                #[doc = include_str!("../../content/learn/book/graphics/rendering-internals/shader-basics/_index.md")]
                mod shader_basics {}
            }
            #[doc = include_str!("../../content/learn/book/graphics/transforms/_index.md")]
            mod transforms {}
            #[doc = include_str!("../../content/learn/book/graphics/windows/_index.md")]
            mod windows {}
        }

        #[doc = include_str!("../../content/learn/book/input/_index.md")]
        mod input {
            #[doc = include_str!("../../content/learn/book/input/gamepad/_index.md")]
            mod gamepad {}
            #[doc = include_str!("../../content/learn/book/input/basics/_index.md")]
            mod basics {}
            #[doc = include_str!("../../content/learn/book/input/keyboard/_index.md")]
            mod keyboard {}
            #[doc = include_str!("../../content/learn/book/input/mouse/_index.md")]
            mod mouse {}
            #[doc = include_str!("../../content/learn/book/input/touch/_index.md")]
            mod touch {}
        }

        // Not testing migration guides, because of breaking api changes.
        mod migration_guides {}

        #[doc = include_str!("../../content/learn/book/performance-optimizations/_index.md")]
        mod performance_optimizations {
            #[doc = include_str!("../../content/learn/book/performance-optimizations/component-storage/_index.md")]
            mod component_storage {}
            #[doc = include_str!("../../content/learn/book/performance-optimizations/diagnostics-benchmarking/_index.md")]
            mod diagnostics_benchmarking {}
            #[doc = include_str!("../../content/learn/book/performance-optimizations/indexes/_index.md")]
            mod indexes {}
            #[doc = include_str!("../../content/learn/book/performance-optimizations/parallel-iteration/_index.md")]
            mod parallel_iteration {}
        }

        #[doc = include_str!("../../content/learn/book/platforms/_index.md")]
        mod platforms {
            #[doc = include_str!("../../content/learn/book/platforms/android/_index.md")]
            mod android {}
            #[doc = include_str!("../../content/learn/book/platforms/ios/_index.md")]
            mod ios {}
            #[doc = include_str!("../../content/learn/book/platforms/web/_index.md")]
            mod web {}
        }

        #[doc = include_str!("../../content/learn/book/ui/_index.md")]
        mod ui {
            #[doc = include_str!("../../content/learn/book/ui/basics/_index.md")]
            mod basics {}
        }

        #[doc = include_str!("../../content/learn/book/welcome/_index.md")]
        mod welcome {
            #[doc = include_str!("../../content/learn/book/welcome/apps/_index.md")]
            mod apps {}
            #[doc = include_str!("../../content/learn/book/welcome/plugins/_index.md")]
            mod plugins {}
            #[doc = include_str!("../../content/learn/book/welcome/setup/_index.md")]
            mod setup {}
        }
    }
}
