//! This crate is used to validate the rust code of the `bevy` website.
//!
//! It is currently used to validate the rust code of the official `bevy` book.
//! The modules represents the folder structure of the website.

// This prevents Clippy from linting the Markdown of our documentation, we just want it to lint
// code blocks. Without this, Clippy would require front-matter and Tera templates to be surrounded
// by backticks.
#![allow(clippy::doc_markdown)]

mod learn {
    #[doc = include_str!("../../content/learn/quick-start/_index.md")]
    mod quick_start {
        #[doc = include_str!("../../content/learn/quick-start/introduction.md")]
        mod introduction {}
        #[doc = include_str!("../../content/learn/quick-start/getting-started/_index.md")]
        mod getting_started {
            #[doc = include_str!("../../content/learn/quick-start/getting-started/apps.md")]
            mod apps {}
            #[doc = include_str!("../../content/learn/quick-start/getting-started/ecs.md")]
            mod ecs {}
            #[doc = include_str!("../../content/learn/quick-start/getting-started/plugins.md")]
            mod plugins {}
            #[doc = include_str!("../../content/learn/quick-start/getting-started/resources.md")]
            mod resources {}
        }
        #[doc = include_str!("../../content/learn/quick-start/plugin-development.md")]
        mod plugin_development {}
        #[doc = include_str!("../../content/learn/quick-start/troubleshooting.md")]
        mod troubleshooting {}
        #[doc = include_str!("../../content/learn/quick-start/next-steps.md")]
        mod next_steps {}

        #[doc = include_str!("../../content/learn/quick-start/breakout/_index.md")]
        mod breakout {}
        #[doc = include_str!("../../content/learn/quick-start/3d-puzzle-game/_index.md")]
        mod three_dimensional_puzzle_game {}
        #[doc = include_str!("../../content/learn/quick-start/falling-sand/_index.md")]
        mod falling_sand {}
    }
    #[doc = include_str!("../../content/learn/advanced-examples/_index.md")]
    mod advanced_examples {
        #[doc = include_str!("../../content/learn/advanced-examples/sudoku/_index.md")]
        mod sudoku {}
        #[doc = include_str!("../../content/learn/advanced-examples/text-adventure/_index.md")]
        mod text_adventure {}
    }
    #[doc = include_str!("../../content/learn/book/_index.md")]
    mod book {
        #[doc = include_str!("../../content/learn/book/assets/_index.md")]
        mod assets {
            #[doc = include_str!("../../content/learn/book/assets/custom-assets.md")]
            mod custom_assets {}
            #[doc = include_str!("../../content/learn/book/assets/hot-reloading.md")]
            mod hot_reloading {}
            #[doc = include_str!("../../content/learn/book/assets/loading-assets.md")]
            mod loading_assets {}
            #[doc = include_str!("../../content/learn/book/assets/scenes-reflection.md")]
            mod scenes_reflection {}
            #[doc = include_str!("../../content/learn/book/assets/working-with-handles.md")]
            mod working_with_handles {}
        }

        #[doc = include_str!("../../content/learn/book/audio/_index.md")]
        mod audio {
            #[doc = include_str!("../../content/learn/book/audio/audio-basics.md")]
            mod audio_basics {}
        }

        #[doc = include_str!("../../content/learn/book/development-practices/_index.md")]
        mod development_practices {
            #[doc = include_str!("../../content/learn/book/development-practices/boilerplate-reduction.md")]
            mod boilerplate_reduction {}
            #[doc = include_str!("../../content/learn/book/development-practices/error-handling.md")]
            mod error_handling {}
            #[doc = include_str!("../../content/learn/book/development-practices/fast-compiles.md")]
            mod fast_compiles {}
            #[doc = include_str!("../../content/learn/book/development-practices/organizing-your-code.md")]
            mod organizing_your_code {}
            #[doc = include_str!("../../content/learn/book/development-practices/testing.md")]
            mod testing {}
        }

        #[doc = include_str!("../../content/learn/book/ecs/_index.md")]
        mod ecs {
            #[doc = include_str!("../../content/learn/book/ecs/change-detection.md")]
            mod change_detection {}
            #[doc = include_str!("../../content/learn/book/ecs/commands.md")]
            mod commands {}
            #[doc = include_str!("../../content/learn/book/ecs/entities-components.md")]
            mod entities_components {}
            #[doc = include_str!("../../content/learn/book/ecs/exclusive-world-access.md")]
            mod exclusive_world_access {}
            #[doc = include_str!("../../content/learn/book/ecs/filtering-queries.md")]
            mod filtering_queries {}
            #[doc = include_str!("../../content/learn/book/ecs/generic-systems.md")]
            mod generic_systems {}
            #[doc = include_str!("../../content/learn/book/ecs/resources.md")]
            mod resources {}
            #[doc = include_str!("../../content/learn/book/ecs/systems-queries.md")]
            mod systems_queries {}
        }

        #[doc = include_str!("../../content/learn/book/game-logic/_index.md")]
        mod game_logic {
            #[doc = include_str!("../../content/learn/book/game-logic/async-tasks.md")]
            mod async_tasks {}
            #[doc = include_str!("../../content/learn/book/game-logic/custom-runners-headless-operations.md")]
            mod custom_runners_headless_operation {}
            #[doc = include_str!("../../content/learn/book/game-logic/events.md")]
            mod events {}
            #[doc = include_str!("../../content/learn/book/game-logic/run-criteria.md")]
            mod run_criteria {}
            #[doc = include_str!("../../content/learn/book/game-logic/states.md")]
            mod states {}
            #[doc = include_str!("../../content/learn/book/game-logic/system-ordering.md")]
            mod system_ordering {}
            #[doc = include_str!("../../content/learn/book/game-logic/time-timers.md")]
            mod time_timers {}
        }

        #[doc = include_str!("../../content/learn/book/getting-started/_index.md")]
        mod getting_started {
            #[doc = include_str!("../../content/learn/book/getting-started/apps-worlds.md")]
            mod apps_worlds {}
            #[doc = include_str!("../../content/learn/book/getting-started/bevy-community.md")]
            mod bevy_community {}
            #[doc = include_str!("../../content/learn/book/getting-started/installation.md")]
            mod installation {}
            #[doc = include_str!("../../content/learn/book/getting-started/modular-plugins.md")]
            mod modular_plugins {}
            #[doc = include_str!("../../content/learn/book/getting-started/why-bevy.md")]
            mod why_bevy {}
        }

        #[doc = include_str!("../../content/learn/book/graphics/_index.md")]
        mod graphics {
            #[doc = include_str!("../../content/learn/book/graphics/2d/_index.md")]
            mod two_dimensional {
                #[doc = include_str!("../../content/learn/book/graphics/2d/sprite-sheets.md")]
                mod sprite_sheets {}
                #[doc = include_str!("../../content/learn/book/graphics/2d/sprites.md")]
                mod sprites {}
            }
            #[doc = include_str!("../../content/learn/book/graphics/3d/_index.md")]
            mod three_dimensional {
                #[doc = include_str!("../../content/learn/book/graphics/3d/meshes.md")]
                mod meshes {}
                #[doc = include_str!("../../content/learn/book/graphics/3d/pbr.md")]
                mod pbr {}
            }
            #[doc = include_str!("../../content/learn/book/graphics/cameras.md")]
            mod cameras {}
            #[doc = include_str!("../../content/learn/book/graphics/parent-child-hierarchy.md")]
            mod parent_child_hierarchy {}
            #[doc = include_str!("../../content/learn/book/graphics/rendering-internals/_index.md")]
            mod rendering_internals {
                #[doc = include_str!("../../content/learn/book/graphics/rendering-internals/shader-basics.md")]
                mod shader_basics {}
            }
            #[doc = include_str!("../../content/learn/book/graphics/transforms.md")]
            mod transforms {}
            #[doc = include_str!("../../content/learn/book/graphics/windows.md")]
            mod windows {}
        }

        #[doc = include_str!("../../content/learn/book/input/_index.md")]
        mod input {
            #[doc = include_str!("../../content/learn/book/input/gamepad.md")]
            mod gamepad {}
            #[doc = include_str!("../../content/learn/book/input/input-basics.md")]
            mod input_basics {}
            #[doc = include_str!("../../content/learn/book/input/keyboard.md")]
            mod keyboard {}
            #[doc = include_str!("../../content/learn/book/input/mouse.md")]
            mod mouse {}
            #[doc = include_str!("../../content/learn/book/input/touch.md")]
            mod touch {}
        }

        // Not testing migration guides, because of breaking API changes.
        mod migration_guides {}

        #[doc = include_str!("../../content/learn/book/performance-optimizations/_index.md")]
        mod performance_optimizations {
            #[doc = include_str!("../../content/learn/book/performance-optimizations/component-storage.md")]
            mod component_storage {}
            #[doc = include_str!("../../content/learn/book/performance-optimizations/diagnostics-benchmarking.md")]
            mod diagnostics_benchmarking {}
            #[doc = include_str!("../../content/learn/book/performance-optimizations/indexes.md")]
            mod indexes {}
            #[doc = include_str!("../../content/learn/book/performance-optimizations/parallel-iteration.md")]
            mod parallel_iteration {}
        }

        #[doc = include_str!("../../content/learn/book/platforms/_index.md")]
        mod platforms {
            #[doc = include_str!("../../content/learn/book/platforms/android.md")]
            mod android {}
            #[doc = include_str!("../../content/learn/book/platforms/ios.md")]
            mod ios {}
            #[doc = include_str!("../../content/learn/book/platforms/web.md")]
            mod web {}
        }

        #[doc = include_str!("../../content/learn/book/ui/_index.md")]
        mod ui {
            #[doc = include_str!("../../content/learn/book/ui/ui-basics.md")]
            mod ui_basics {}
        }
    }
}
