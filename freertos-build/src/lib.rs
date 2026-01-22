#![doc = include_str!("../README.md")]

pub mod prelude;

use cc::Build;
use fugit::HertzU32;
use prelude::*;
use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const ENV_KEY_CRATE_DIR: &str = "DEP_FREERTOS_CRATE_DIR";

#[derive(Debug, Clone)]
pub struct Builder {
    freertos_dir: PathBuf,
    config_dir: PathBuf,
    user_config_dir: Option<PathBuf>,
    shim_file: PathBuf,
    freertos_port: Option<PathBuf>,
    freertos_port_base: Option<PathBuf>,
    /// name of the heap_?.c file
    heap_c: PathBuf,
    cc: Build,
    cpu_clock: HertzU32,
    heap_size: usize,
    /// in words
    minimal_stack_size: usize,
    max_priorities: u8,
    timer_task_config: Option<TimerTaskConfig>,
    use_preemption: bool,
    idle_should_yield: Option<bool>,
    interrupt_priority_bits: Option<InterruptPriorityBits>,
    interrupt_priority: Option<InterruptPriority>,
    max_task_name_len: Option<usize>,
    queue_registry_size: Option<usize>,
    check_for_stack_overflow: Option<u8>,
}

#[derive(Debug, Clone, Copy)]
struct TimerTaskConfig {
    priority: u8,
    queue_length: usize,
    stack_depth: usize,
}

#[derive(Debug, Clone, Copy)]
struct InterruptPriorityBits {
    bits: u8,
    lowest_priority: u32,
    max_syscall_priority: u32,
}

#[derive(Debug, Clone, Copy)]
struct InterruptPriority {
    lowest_priority: u32,
    max_syscall_priority: u32,
}

#[derive(Debug)]
pub enum Error {
    /// More explanation of error that occurred.
    Message(String),
}

impl Error {
    fn new(message: &str) -> Self {
        Self::Message(message.to_owned())
    }
}

impl Default for Builder {
    fn default() -> Self {
        let crate_dir = PathBuf::from(env::var(ENV_KEY_CRATE_DIR).unwrap());

        Self {
            freertos_dir: crate_dir.join("FreeRTOS-Kernel"),
            shim_file: crate_dir.join("src/freertos/shim.c"),
            config_dir: crate_dir.join("src/config"),
            user_config_dir: None,
            freertos_port: None,
            freertos_port_base: None,
            cc: cc::Build::new(),
            heap_c: PathBuf::from("heap_4.c"),
            cpu_clock: 0.Hz(),
            heap_size: 16 * 1024,
            minimal_stack_size: 80,
            max_priorities: 5,
            timer_task_config: None,
            use_preemption: true,
            idle_should_yield: None,
            interrupt_priority_bits: None,
            interrupt_priority: None,
            max_task_name_len: None,
            queue_registry_size: None,
            check_for_stack_overflow: None,
        }
    }
}

macro_rules! set_define {
    ($cc:ident, $def:expr, $v:expr) => {
        $cc.define($def, $v.to_string().as_str());
    };
    (bool, $cc:ident, $def:expr, $v:expr) => {
        let v = if $v { 1 } else { 0 };
        $cc.define($def, v.to_string().as_str());
    };
}

impl Builder {
    /// Construct a new instance of a blank set of configuration.
    ///
    /// This builder is finished with the [`compile`] function.
    ///
    /// [`compile`]: struct.Build.html#method.compile
    pub fn new() -> Builder {
        Self::default()
    }

    /// Set the path to FreeRTOS-Kernel source files
    pub fn freertos_kernel<P: AsRef<Path>>(&mut self, path: P) {
        self.freertos_dir = path.as_ref().to_path_buf();
    }

    /// Set the path to the directory of `FreeRTOSConfig.h`
    pub fn freertos_config_dir<P: AsRef<Path>>(&mut self, path: P) {
        self.config_dir = path.as_ref().to_path_buf();
    }

    /// Set the path to the directory of `UserConfig.h`
    pub fn user_config_dir<P: AsRef<Path>>(&mut self, path: P) {
        self.user_config_dir = Some(path.as_ref().to_path_buf());
    }

    /// Set the path to shim.c
    pub fn shim_file<P: AsRef<Path>>(&mut self, path: P) {
        self.shim_file = path.as_ref().to_path_buf();
    }

    /// Returns a list of all FreeRTOS source files
    fn freertos_files(&self) -> Vec<PathBuf> {
        let files: Vec<_> = WalkDir::new(self.freertos_dir.as_path())
            .follow_links(false)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter_map(|entry| {
                let f_name = entry.path().to_str().unwrap();

                if f_name.ends_with(".c") {
                    return Some(entry.path().to_owned());
                }
                None
            })
            .collect();
        files
    }
    fn freertos_port_files(&self) -> Vec<PathBuf> {
        let files: Vec<_> = WalkDir::new(self.get_freertos_port_dir())
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter_map(|entry| {
                match entry
                    .path()
                    .extension()
                    .map(|s| s.to_string_lossy())
                    .as_ref()
                    .map(|s| s.as_ref())
                {
                    Some("c" | "s" | "S") => Some(entry.path().to_owned()),
                    _ => None,
                }
            })
            .collect();
        files
    }

    /// Set the heap_?.c file to use from the "/portable/MemMang/" folder.
    /// heap_1.c ... heap_5.c (Default: heap_4.c)
    /// see also: https://www.freertos.org/a00111.html
    pub fn heap<P: AsRef<Path>>(&mut self, file_name: P) {
        self.heap_c = file_name.as_ref().to_path_buf();
    }

    /// Access to the underlining cc::Build instance to further customize the build.
    pub fn get_cc(&mut self) -> &mut Build {
        &mut self.cc
    }

    pub fn cpu_clock(&mut self, clock: HertzU32) {
        self.cpu_clock = clock;
    }

    pub fn heap_size(&mut self, size: usize) {
        self.heap_size = size;
    }

    /// in words
    pub fn minimal_stack_size(&mut self, size: usize) {
        self.minimal_stack_size = size;
    }

    pub fn max_task_priorities(&mut self, val: u8) {
        self.max_priorities = val;
    }

    /// http://www.freertos.org/Configuring-a-real-time-RTOS-application-to-use-software-timers.html
    pub fn use_timer_task(&mut self, priority: u8, queue_length: usize, stack_depth: usize) {
        self.timer_task_config = Some(TimerTaskConfig {
            priority,
            queue_length,
            stack_depth,
        });
    }

    pub fn use_preemption(&mut self, v: bool) {
        self.use_preemption = v;
    }

    pub fn idle_should_yield(&mut self, v: bool) {
        self.idle_should_yield = Some(v);
    }

    /// http://www.FreeRTOS.org/RTOS-Cortex-M3-M4.html
    pub fn interrupt_priority_bits(
        &mut self,
        bits: u8,
        max_syscall_priority: u32,
        lowest_priority: u32,
    ) {
        self.interrupt_priority_bits = Some(InterruptPriorityBits {
            bits,
            lowest_priority,
            max_syscall_priority,
        });
    }

    pub fn interrupt_priority(&mut self, max_syscall_priority: u32, lowest_priority: u32) {
        self.interrupt_priority = Some(InterruptPriority {
            lowest_priority,
            max_syscall_priority,
        });
    }

    pub fn max_task_name_len(&mut self, v: usize) {
        self.max_task_name_len = Some(v);
    }

    pub fn queue_registry_size(&mut self, v: usize) {
        self.queue_registry_size = Some(v);
    }

    pub fn check_for_stack_overflow(&mut self, v: u8) {
        self.check_for_stack_overflow = Some(v)
    }

    fn freertos_include_dir(&self) -> PathBuf {
        self.freertos_dir.join("include")
    }

    /// set the freertos port dir relativ to the FreeRTOS/Source/portable directory
    /// e.g. "GCC/ARM_CM33_NTZ/non_secure"
    ///
    /// If not set it will be detected based on the current build target (not many targets supported yet).
    pub fn freertos_port<P: AsRef<Path>>(&mut self, port_dir: P) {
        self.freertos_port = Some(port_dir.as_ref().to_path_buf());
    }

    fn get_freertos_port_dir(&self) -> PathBuf {
        let base = self.get_freertos_port_base();
        if self.freertos_port.is_some() {
            return base.join(self.freertos_port.as_ref().unwrap());
        }

        let target = env::var("TARGET").unwrap_or_default();
        let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default(); // msvc, gnu, ...
        //let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default(); // unix, windows
        let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default(); // x86_64
        let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default(); // none, windows, linux, macos
        let port = match (
            target.as_str(),
            target_arch.as_str(),
            target_os.as_str(),
            target_env.as_str(),
        ) {
            (_, "x86_64", "windows", _) => "MSVC-MingW",
            (_, "x86_64", "linux", "gnu") => "ThirdParty/GCC/Posix",
            ("thumbv7m-none-eabi", _, _, _) => "GCC/ARM_CM3",
            ("thumbv7em-none-eabi", _, _, _) => "GCC/ARM_CM3", // M4 cores without FPU use M3
            ("thumbv7em-none-eabihf", _, _, _) => "GCC/ARM_CM4F",
            // TODO We should support feature "trustzone"
            ("thumbv8m.main-none-eabi", _, _, _) => "GCC/ARM_CM33_NTZ/non_secure",
            ("thumbv8m.main-none-eabihf", _, _, _) => "GCC/ARM_CM33_NTZ/non_secure",
            _ => {
                panic!(
                    "Unknown target: '{}', from TARGET environment variable.",
                    target
                );
            }
        };
        base.join(port)
    }

    pub fn freertos_port_base<P: AsRef<Path>>(&mut self, base_dir: P) {
        self.freertos_port_base = Some(base_dir.as_ref().to_path_buf());
    }

    fn get_freertos_port_base(&self) -> PathBuf {
        if let Some(base) = &self.freertos_port_base {
            base.clone()
        } else {
            PathBuf::from(&self.freertos_dir).join("portable")
        }
    }

    fn heap_c_file(&self) -> PathBuf {
        self.freertos_dir
            .join("portable/MemMang")
            .join(&self.heap_c)
    }

    /// Check that all required files and paths exist
    fn verify_paths(&self) -> Result<(), Error> {
        if !self.freertos_dir.is_dir() {
            return Err(Error::new(&format!(
                "Directory freertos_dir does not exist: {}",
                self.freertos_dir.to_str().unwrap()
            )));
        }
        let port_dir = self.get_freertos_port_dir();
        if !port_dir.is_dir() {
            return Err(Error::new(&format!(
                "Directory freertos_port_dir does not exist: {}",
                port_dir.to_str().unwrap()
            )));
        }

        let include_dir = self.freertos_include_dir();
        if !include_dir.is_dir() {
            return Err(Error::new(&format!(
                "Directory freertos_include_dir does not exist: {}",
                include_dir.to_str().unwrap()
            )));
        }

        // The heap implementation
        let heap_c = self.heap_c_file();
        if !heap_c.is_file() {
            return Err(Error::new(&format!(
                "File heap_?.c does not exist: {}",
                heap_c.to_str().unwrap()
            )));
        }

        // Make sure FreeRTOSConfig.h exists in freertos_config_dir
        if !self.config_dir.join("FreeRTOSConfig.h").is_file() {
            return Err(Error::new(&format!(
                "File FreeRTOSConfig.h does not exist in the directory: {}",
                self.config_dir.to_str().unwrap()
            )));
        }

        if let Some(dir) = &self.user_config_dir {
            if !dir.join("UserConfig.h").is_file() {
                return Err(Error::new(&format!(
                    "File UserConfig.h does not exist in the directory: {}",
                    dir.to_str().unwrap()
                )));
            }
        }

        // Add the freertos shim.c
        if !self.shim_file.is_file() {
            return Err(Error::new(&format!(
                "File freertos_shim '{}' does not exist, missing freertos dependency?",
                self.shim_file.to_str().unwrap()
            )));
        }

        Ok(())
    }

    pub fn compile(&self) -> Result<(), Error> {
        let mut cc = self.cc.clone();

        self.verify_paths()?;

        add_include_with_rerun(&mut cc, self.freertos_include_dir()); // FreeRTOS header files
        add_include_with_rerun(&mut cc, self.get_freertos_port_dir()); // FreeRTOS port header files (e.g. portmacro.h)
        add_include_with_rerun(&mut cc, &self.config_dir); // FreeRTOSConfig.h
        if let Some(dir) = &self.user_config_dir {
            set_define!(cc, "__HAS_USER_CONFIG", 1);
            add_include_with_rerun(&mut cc, dir); // User's UserConfig.h
            println!("cargo:rerun-if-env-changed={}", dir.to_str().unwrap());
        }

        add_build_files_with_rerun(&mut cc, self.freertos_files()); // Non-port C files
        add_build_files_with_rerun(&mut cc, self.freertos_port_files()); // Port C files
        add_build_file_with_rerun(&mut cc, &self.shim_file); // Shim C file
        add_build_file_with_rerun(&mut cc, self.heap_c_file()); // Heap C file

        if self.cpu_clock.raw() > 0 {
            set_define!(cc, "configCPU_CLOCK_HZ", self.cpu_clock.raw());
        }
        set_define!(cc, "configMINIMAL_STACK_SIZE", self.minimal_stack_size);
        set_define!(cc, "configTOTAL_HEAP_SIZE", self.heap_size);
        set_define!(cc, "configMAX_PRIORITIES", self.max_priorities);
        if let Some(config) = &self.timer_task_config {
            set_define!(cc, "configUSE_TIMERS", 1);
            set_define!(cc, "configTIMER_TASK_PRIORITY", config.priority);
            set_define!(cc, "configTIMER_QUEUE_LENGTH", config.queue_length);
            set_define!(cc, "configTIMER_TASK_STACK_DEPTH", config.stack_depth);
        }
        set_define!(bool, cc, "configUSE_PREEMPTION", self.use_preemption);
        if let Some(v) = self.idle_should_yield {
            set_define!(bool, cc, "configIDLE_SHOULD_YIELD", v);
        }
        if let Some(config) = self.interrupt_priority_bits {
            set_define!(
                cc,
                "configKERNEL_INTERRUPT_PRIORITY",
                config.lowest_priority << (8 - config.bits)
            );
            set_define!(
                cc,
                "configMAX_SYSCALL_INTERRUPT_PRIORITY",
                config.max_syscall_priority << (8 - config.bits)
            );
        }
        if let Some(config) = self.interrupt_priority {
            set_define!(
                cc,
                "configKERNEL_INTERRUPT_PRIORITY",
                config.lowest_priority
            );
            set_define!(
                cc,
                "configMAX_SYSCALL_INTERRUPT_PRIORITY",
                config.max_syscall_priority
            );
        }
        if let Some(v) = self.max_task_name_len {
            set_define!(cc, "configMAX_TASK_NAME_LEN", v);
        }
        if let Some(v) = self.queue_registry_size {
            set_define!(cc, "configQUEUE_REGISTRY_SIZE", v);
        }
        if let Some(v) = self.check_for_stack_overflow {
            set_define!(cc, "configCHECK_FOR_STACK_OVERFLOW", v);
        }
        setup_all_define(&mut cc);

        println!(
            "cargo:rerun-if-env-changed={}",
            self.freertos_dir.to_str().unwrap()
        );
        println!(
            "cargo:rerun-if-env-changed={}",
            self.config_dir.to_str().unwrap()
        );
        println!(
            "cargo:rerun-if-env-changed={}",
            self.shim_file.to_str().unwrap()
        );

        cc.try_compile("freertos")
            .map_err(|e| Error::new(&format!("{}", e)))?;

        Ok(())
    }

    /// Add a single file to the build. This also tags the file with cargo:rerun-if-changed so that cargo will re-run
    /// the build script if the file changes. If you don't want this additional behavior, use get_cc().file() to
    /// directly add a file to the build instead.
    pub fn add_build_file<P: AsRef<Path>>(&mut self, file: P) {
        add_build_file_with_rerun(self.get_cc(), file);
    }

    /// Add multiple files to the build. This also tags the files with cargo:rerun-if-changed so that cargo will re-run
    /// the build script if the files change. If you don't want this additional behavior, use get_cc().files() to
    /// directly add files to the build instead.
    pub fn add_build_files<P>(&mut self, files: P)
    where
        P: IntoIterator,
        P::Item: AsRef<Path>,
    {
        add_build_files_with_rerun(self.get_cc(), files);
    }
}

fn add_build_file_with_rerun<P: AsRef<Path>>(build: &mut Build, file: P) {
    build.file(&file);
    println!("cargo:rerun-if-changed={}", file.as_ref().display());
}

fn add_build_files_with_rerun<P>(build: &mut Build, files: P)
where
    P: IntoIterator,
    P::Item: AsRef<Path>,
{
    for file in files.into_iter() {
        add_build_file_with_rerun(build, file);
    }
}

fn add_include_with_rerun<P: AsRef<Path>>(build: &mut Build, dir: P) {
    build.include(&dir);

    WalkDir::new(&dir)
        .follow_links(false)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .for_each(|entry| {
            let f_name = entry.path();
            if f_name.extension() == Some(OsStr::new("h")) {
                println!("cargo:rerun-if-changed={}", f_name.display());
            }
        });
}

fn setup_all_define(cc: &mut cc::Build) {
    sync_define(cc, "__IS_CORTEX_M");
    sync_define(cc, "INCLUDE_vTaskDelete");
    sync_define(cc, "INCLUDE_vTaskDelayUntil");
    sync_define(cc, "INCLUDE_uxTaskGetStackHighWaterMark");
    sync_define(cc, "INCLUDE_HeapFreeSize");
    sync_define(cc, "INCLUDE_vTaskSuspend");
    sync_define(cc, "configUSE_RECURSIVE_MUTEXES");
    sync_define(cc, "configUSE_COUNTING_SEMAPHORES");
    sync_define(cc, "configUSE_TRACE_FACILITY");
}

fn sync_define(cc: &mut cc::Build, def: &str) {
    let v = "DEP_FREERTOS_DEF_".to_string() + &def.to_uppercase();
    let v_string = env::var(v).unwrap_or("0".to_string());
    set_define!(cc, def, v_string);
}

#[test]
fn test_paths() {
    unsafe { env::set_var("FREERTOS_SRC", "some/path") };
    unsafe { env::set_var("TARGET", "thumbv8m.main-none-eabihf") };
    let b = Builder::new();
    assert_eq!(b.freertos_dir.to_str().unwrap(), "some/path");
}
