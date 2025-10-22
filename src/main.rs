use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::fs::File;
use std::io::{self, Write};

/*
Gaurav Sablok
codeprog@icloud.com
*/

#[derive(Clone)]
struct ModuleCategory {
    name: String,
    modules: Vec<String>,
}

#[derive(Default)]
struct App {
    username: String,
    nodes: String,
    tasks: String,
    cpu: String,
    memory: String,
    time: String,
    workdir: String,
    email: String,
    command: String,
    exportpath: String,
    additionaltext: String,
    selected_category: usize,
    selected_module: usize,
    categories: Vec<ModuleCategory>,
    category_state: ListState,
    module_state: ListState,
    input_field: usize,
    dropdown_open: bool,
    running: bool,
}

impl App {
    fn new() -> App {
        let categories = vec![
            ModuleCategory {
                name: "bio".to_string(),
                modules: vec![
                    "bio/AUGUSTUS/3.4.0-foss-2020b".to_string(),
                    "bio/AlphaFold/2.0.1-fosscuda-2020b".to_string(),
                    "bio/AlphaFold/2.1.1-fosscuda-2020b".to_string(),
                    "bio/AlphaFold/2.2.0-fosscuda-2020b".to_string(),
                    "bio/BCFtools/1.11-GCC-10.2.0".to_string(),
                    "bio/BLAST/2.11.0-Linux_x86_64".to_string(),
                    "bio/BLAST/2.12.0-Linux_x86_64".to_string(),
                    "bio/BLAST+/2.11.0-gompi-2020b".to_string(),
                    "bio/BUSCO/4.1.4-foss-2020b".to_string(),
                    "bio/BUSCO/5.0.0-foss-2020b".to_string(),
                    "bio/BUSCO/5.1.2-foss-2020b".to_string(),
                    "bio/BWA/0.7.17-GCC-10.2.0".to_string(),
                    "bio/BamTools/2.5.1-GCC-10.2.0".to_string(),
                    "bio/Beast/2.5.1-foss-2018b".to_string(),
                    "bio/BioPerl/1.7.8-GCCcore-10.2.0".to_string(),
                    "bio/Biopython/1.78-foss-2020b".to_string(),
                    "bio/Biopython/1.78-fosscuda-2020b".to_string(),
                    "bio/Bowtie/1.2.2-foss-2018b".to_string(),
                    "bio/Bowtie/1.3.0-GCC-10.2.0".to_string(),
                    "bio/Bowtie2/2.3.4.2-foss-2018b".to_string(),
                    "bio/Bowtie2/2.4.2-GCC-10.2.0".to_string(),
                    "bio/CAFE5/5.0.0-GCC-10.2.0".to_string(),
                    "bio/CD-HIT/4.8.1-GCC-10.2.0".to_string(),
                    "bio/Clustal-Omega/1.2.4-foss-2018b".to_string(),
                    "bio/DendroPy/4.5.2-GCCcore-10.2.0".to_string(),
                    "bio/EMBOSS/6.6.0-foss-2018b".to_string(),
                    "bio/Exonerate/2.4.0-GCC-10.2.0".to_string(),
                    "bio/FASTX-Toolkit/0.0.14-GCC-9.3.0".to_string(),
                    "bio/FLASH/2.2.00-foss-2018b".to_string(),
                    "bio/FastQC/0.11.9-Java-1.8".to_string(),
                    "bio/GROMACS/2021.2-fosscuda-2020b".to_string(),
                    "bio/GROMACS/2021.2-fosscuda-2020b-znver1".to_string(),
                    "bio/Gctf/1.06".to_string(),
                    "bio/GenomeThreader/1.7.3-Linux_x86_64-64bit".to_string(),
                    "bio/GenomeTools/1.6.1-GCC-10.2.0".to_string(),
                    "bio/HH-suite/3.3.0-gompic-2020b".to_string(),
                    "bio/HISAT2/2.2.1-gompi-2020b".to_string(),
                    "bio/HMMER/3.3.2-gompi-2020b".to_string(),
                    "bio/HMMER/3.3.2-gompic-2020b".to_string(),
                    "bio/HTSlib/1.11-GCC-10.2.0".to_string(),
                    "bio/HTSlib/1.12-GCC-10.2.0".to_string(),
                    "bio/HyPhy/2.5.1-gompi-2020b".to_string(),
                    "bio/IMAGIC/20100209".to_string(),
                    "bio/IMAGIC/20101031".to_string(),
                    "bio/InterProScan/5.28-67.0-foss-2020b".to_string(),
                    "bio/InterProScan/5.55-88.0-foss-2020b".to_string(),
                    "bio/Jellyfish/2.3.0-GCC-8.3.0".to_string(),
                    "bio/Kalign/3.3.1-GCCcore-10.2.0".to_string(),
                    "bio/Kent_tools/411-GCC-10.2.0".to_string(),
                    "bio/Kraken/1.1.1-GCCcore-10.2.0".to_string(),
                    "bio/Kraken2/2.1.1-gompi-2020b".to_string(),
                    "bio/LTR_retriever/2.9.0-foss-2020b".to_string(),
                    "bio/MAFFT/7.453-GCC-9.3.0-with-extensions".to_string(),
                    "bio/MAFFT/7.475-gompi-2020b-with-extensions".to_string(),
                    "bio/MAKER/3.01.03-foss-2020b".to_string(),
                    "bio/MUMmer/4.0.0beta2-GCCcore-10.2.0".to_string(),
                    "bio/MaxQuant/2.0.3.0-foss-2018b-aot".to_string(),
                    "bio/MaxQuant/2.0.3.0-foss-2018b".to_string(),
                    "bio/MetaEuk/4-GCC-10.2.0".to_string(),
                    "bio/Mothur/1.46.1-foss-2020b".to_string(),
                    "bio/MotionCor2/1.4.2-GCCcore-10.2.0".to_string(),
                    "bio/MrBayes/3.2.7-GCC-10.2.0".to_string(),
                    "bio/NGS/2.10.9-GCCcore-10.2.0".to_string(),
                    "bio/OMA/2.5.0-GCCcore-10.2.0".to_string(),
                    "bio/PAML/4.9j-GCCcore-9.3.0".to_string(),
                    "bio/PHYLIP/3.697-GCC-9.3.0".to_string(),
                    "bio/PLINK/1.9b_6.21-x86_64".to_string(),
                    "bio/PRANK/170427-GCC-9.3.0".to_string(),
                    "bio/RAxML/8.2.12-foss-2020b-pthreads-avx2".to_string(),
                    "bio/RAxML/8.2.12-gompi-2020a-hybrid-avx2".to_string(),
                    "bio/RECON/1.08-GCC-10.2.0".to_string(),
                    "bio/RELION/3.0_beta.2018.08.02-fosscuda-2020b".to_string(),
                    "bio/RELION/3.1.1-fosscuda-2020b".to_string(),
                    "bio/RELION/4.0.0-fosscuda-2020b".to_string(),
                    "bio/RELION/4git1fb5b8f-fosscuda-2020b".to_string(),
                    "bio/RELION/4git44c8b38-fosscuda-2018b".to_string(),
                    "bio/RELION/4git44c8b38-fosscuda-2020b".to_string(),
                    "bio/RELION/4git44c8b38-2-fosscuda-2020b".to_string(),
                    "bio/RMBlast/2.11.0-gompi-2020b".to_string(),
                    "bio/RSEM/1.3.3-foss-2020b".to_string(),
                    "bio/RepeatMasker/4.1.2-p1-foss-2020b".to_string(),
                    "bio/RepeatModeler/2.0.2a-foss-2020b".to_string(),
                    "bio/RepeatScout/1.0.6-GCC-10.2.0".to_string(),
                    "bio/ResMap/1.1.4-Linux_x86_64".to_string(),
                    "bio/SAMtools/1.11-GCC-10.2.0".to_string(),
                    "bio/SAMtools/1.12-GCC-10.2.0".to_string(),
                    "bio/SAMtools/1.15-GCC-10.2.0".to_string(),
                    "bio/SEPP/4.4.0-foss-2020b".to_string(),
                    "bio/SNAP-HMM/20190603-GCC-10.2.0".to_string(),
                    "bio/SRA-Toolkit/2.10.9-gompi-2020b".to_string(),
                    "bio/SRAssembler/1.0.0-foss-2020b".to_string(),
                    "bio/STAR/2.7.7a-GCC-10.2.0".to_string(),
                    "bio/Salmon/1.4.0-gompi-2020b".to_string(),
                    "bio/Stacks/2.54-foss-2020a".to_string(),
                    "bio/TCoffee/13.45.0-GCCcore-10.2.0".to_string(),
                    "bio/TRF/4.09-linux64".to_string(),
                    "bio/TRF/4.09.1-GCCcore-10.2.0".to_string(),
                    "bio/TWL-NINJA/0.97-cluster_only-GCC-10.2.0".to_string(),
                    "bio/Trimmomatic/0.39-Java-1.8".to_string(),
                    "bio/Trinity/2.8.4-foss-2018b".to_string(),
                    "bio/Vmatch/2.3.1-Linux_x86_64-64bit".to_string(),
                    "bio/Vmatch/2.3.1".to_string(),
                    "bio/angsd/0.935-GCC-10.2.0".to_string(),
                    "bio/beta-psmc/git1e9ab32-foss-2020b".to_string(),
                    "bio/ctffind/4.1.14-foss-2020b".to_string(),
                    "bio/ctffind/4.1.14-fosscuda-2020b".to_string(),
                    "bio/ctffind/4.1.14-Linux_x86_64".to_string(),
                    "bio/cutadapt/2.10-GCCcore-9.3.0-Python-3.8.2".to_string(),
                    "bio/dssp/2.3.0-GCC-10.2.0".to_string(),
                    "bio/fastStructure/1.0-foss-2020b-Python-2.7.18".to_string(),
                    "bio/hssp/3.1.5-GCC-10.2.0".to_string(),
                    "bio/mrcfile/1.3.0-fosscuda-2020b".to_string(),
                    "bio/ncbi-vdb/2.10.9-gompi-2020b".to_string(),
                    "bio/phyx/1.3-foss-2020b".to_string(),
                    "bio/prodigal/2.6.3-GCCcore-10.2.0".to_string(),
                    "bio/psmc/0.6.5-foss-2020b".to_string(),
                    "bio/smcpp/1.15.2-foss-2020b".to_string(),
                ],
            },
            ModuleCategory {
                name: "chem".to_string(),
                modules: vec![
                    "chem/LAMMPS/3Mar2020-foss-2020a-Python-3.8.2-kokkos".to_string(),
                    "chem/PLUMED/2.6.0-foss-2020a-Python-3.8.2".to_string(),
                    "chem/GROMACS/2020.4-foss-2020a".to_string(),
                ],
            },
            ModuleCategory {
                name: "compiler".to_string(),
                modules: vec![
                    "compiler/Clang/11.0.1-gcccuda-2020b".to_string(),
                    "compiler/GCC/7.3.0-2.30".to_string(),
                    "compiler/GCC/8.3.0".to_string(),
                    "compiler/GCC/9.3.0".to_string(),
                    "compiler/GCC/10.2.0".to_string(),
                    "compiler/GCCcore/7.3.0".to_string(),
                    "compiler/GCCcore/8.3.0".to_string(),
                    "compiler/GCCcore/9.3.0".to_string(),
                    "compiler/GCCcore/10.2.0".to_string(),
                    "compiler/Go/1.13.1".to_string(),
                    "compiler/LLVM/6.0.0-GCCcore-7.3.0".to_string(),
                    "compiler/LLVM/9.0.1-GCCcore-9.3.0".to_string(),
                    "compiler/LLVM/10.0.1-GCCcore-10.2.0".to_string(),
                    "compiler/LLVM/11.0.0-GCCcore-10.2.0".to_string(),
                ],
            },
            ModuleCategory {
                name: "data".to_string(),
                modules: vec![
                    "data/DB_File/1.855-GCCcore-10.2.0".to_string(),
                    "data/GDAL/3.2.1-foss-2020b".to_string(),
                    "data/GDAL/3.2.1-fosscuda-2020b".to_string(),
                    "data/HDF/4.2.15-GCCcore-10.2.0".to_string(),
                    "data/HDF5/1.10.2-fosscuda-2018b".to_string(),
                    "data/HDF5/1.10.6-gompi-2020a".to_string(),
                    "data/HDF5/1.10.7-gompi-2020b".to_string(),
                    "data/HDF5/1.10.7-gompic-2020b".to_string(),
                    "data/LAME/3.100-GCCcore-9.3.0".to_string(),
                    "data/LAME/3.100-GCCcore-10.2.0".to_string(),
                    "data/MariaDB/10.5.8-GCC-10.2.0".to_string(),
                    "data/PostgreSQL/13.2-GCCcore-10.2.0".to_string(),
                    "data/XML-LibXML/2.0206-GCCcore-10.2.0".to_string(),
                    "data/XML-Parser/2.44_01-GCCcore-7.3.0-Perl-5.28.0".to_string(),
                    "data/dask/2021.2.0-foss-2020b".to_string(),
                    "data/dask/2021.2.0-fosscuda-2020b".to_string(),
                    "data/h5py/2.10.0-foss-2020a-Python-3.8.2".to_string(),
                    "data/h5py/3.1.0-foss-2020b".to_string(),
                    "data/netCDF/4.6.1-fosscuda-2018b".to_string(),
                    "data/netCDF/4.7.4-gompi-2020a".to_string(),
                    "data/netCDF/4.7.4-gompi-2020b".to_string(),
                    "data/netCDF/4.7.4-gompic-2020b".to_string(),
                    "data/scikit-learn/0.23.2-fosscuda-2020b".to_string(),
                ],
            },
            ModuleCategory {
                name: "devel".to_string(),
                modules: vec![
                    "devel/Autoconf/2.69-GCCcore-9.3.0".to_string(),
                    "devel/Autoconf/2.69-GCCcore-10.2.0".to_string(),
                    "devel/Automake/1.16.1-GCCcore-7.3.0".to_string(),
                    "devel/Automake/1.16.1-GCCcore-9.3.0".to_string(),
                    "devel/Automake/1.16.2-GCCcore-10.2.0".to_string(),
                    "devel/Autotools/20180311-GCCcore-7.3.0".to_string(),
                    "devel/Autotools/20180311-GCCcore-9.3.0".to_string(),
                    "devel/Autotools/20200321-GCCcore-10.2.0".to_string(),
                    "devel/Bazel/3.7.2-GCCcore-10.2.0".to_string(),
                    "devel/Boost/1.72.0-gompi-2020a".to_string(),
                    "devel/Boost/1.74.0-GCC-10.2.0".to_string(),
                    "devel/CMake/3.11.4-GCCcore-7.3.0".to_string(),
                    "devel/CMake/3.12.1-GCCcore-7.3.0".to_string(),
                    "devel/CMake/3.16.4-GCCcore-9.3.0".to_string(),
                    "devel/CMake/3.18.4-GCCcore-10.2.0".to_string(),
                    "devel/DBus/1.13.6-GCCcore-7.3.0".to_string(),
                    "devel/DBus/1.13.12-GCCcore-9.3.0".to_string(),
                    "devel/DBus/1.13.18-GCCcore-10.2.0".to_string(),
                    "devel/Doxygen/1.8.14-GCCcore-7.3.0".to_string(),
                    "devel/Doxygen/1.8.17-GCCcore-9.3.0".to_string(),
                    "devel/Doxygen/1.8.20-GCCcore-10.2.0".to_string(),
                    "devel/GConf/3.2.6-GCCcore-10.2.0".to_string(),
                    "devel/GObject-Introspection/1.54.1-fosscuda-2018b-Python-2.7.15".to_string(),
                    "devel/GObject-Introspection/1.66.1-GCCcore-10.2.0".to_string(),
                    "devel/JUnit/4.12-Java-1.8".to_string(),
                    "devel/LZO/2.10-GCCcore-10.2.0".to_string(),
                    "devel/LevelDB/1.22-GCCcore-10.2.0".to_string(),
                    "devel/M4/1.4.17".to_string(),
                    "devel/M4/1.4.18-GCCcore-7.3.0".to_string(),
                    "devel/M4/1.4.18-GCCcore-8.3.0".to_string(),
                    "devel/M4/1.4.18-GCCcore-9.3.0".to_string(),
                    "devel/M4/1.4.18-GCCcore-10.2.0".to_string(),
                    "devel/M4/1.4.18".to_string(),
                    "devel/Mako/1.0.7-fosscuda-2018b-Python-2.7.15".to_string(),
                    "devel/Mako/1.1.2-GCCcore-9.3.0".to_string(),
                    "devel/Mako/1.1.3-GCCcore-10.2.0".to_string(),
                    "devel/Maven/3.6.3".to_string(),
                    "devel/PCRE/8.41-GCCcore-7.3.0".to_string(),
                    "devel/PCRE/8.44-GCCcore-9.3.0".to_string(),
                    "devel/PCRE/8.44-GCCcore-10.2.0".to_string(),
                    "devel/PCRE2/10.34-GCCcore-9.3.0".to_string(),
                    "devel/PCRE2/10.35-GCCcore-10.2.0".to_string(),
                    "devel/PyTorch/1.7.1-fosscuda-2020b".to_string(),
                    "devel/Qt5/5.10.1-fosscuda-2018b".to_string(),
                    "devel/Qt5/5.14.1-GCCcore-9.3.0".to_string(),
                    "devel/Qt5/5.14.2-GCCcore-10.2.0".to_string(),
                    "devel/SQLite/3.24.0-GCCcore-7.3.0".to_string(),
                    "devel/SQLite/3.31.1-GCCcore-9.3.0".to_string(),
                    "devel/SQLite/3.33.0-GCCcore-10.2.0".to_string(),
                    "devel/SWIG/4.0.2-GCCcore-10.2.0".to_string(),
                    "devel/ZeroMQ/4.3.2-GCCcore-9.3.0".to_string(),
                    "devel/ZeroMQ/4.3.3-GCCcore-10.2.0".to_string(),
                    "devel/ant/1.10.1-Java-1.8".to_string(),
                    "devel/ant/1.10.5-Java-1.8".to_string(),
                    "devel/ant/1.10.8-Java-11".to_string(),
                    "devel/ant/1.10.9-Java-11".to_string(),
                    "devel/dbus-glib/0.110-GCCcore-10.2.0".to_string(),
                    "devel/flatbuffers/1.12.0-GCCcore-10.2.0".to_string(),
                    "devel/flatbuffers-python/1.12-GCCcore-10.2.0".to_string(),
                    "devel/gflags/2.2.2-GCCcore-10.2.0".to_string(),
                    "devel/glog/0.5.0-GCCcore-10.2.0".to_string(),
                    "devel/gperf/3.1-GCCcore-7.3.0".to_string(),
                    "devel/gperf/3.1-GCCcore-9.3.0".to_string(),
                    "devel/gperf/3.1-GCCcore-10.2.0".to_string(),
                    "devel/intltool/0.51.0-GCCcore-7.3.0-Perl-5.28.0".to_string(),
                    "devel/intltool/0.51.0-GCCcore-9.3.0".to_string(),
                    "devel/intltool/0.51.0-GCCcore-10.2.0".to_string(),
                    "devel/makeinfo/6.7-GCCcore-9.3.0".to_string(),
                    "devel/makeinfo/6.7-GCCcore-10.2.0".to_string(),
                    "devel/ncurses/6.0".to_string(),
                    "devel/ncurses/6.1-GCCcore-7.3.0".to_string(),
                    "devel/ncurses/6.1".to_string(),
                    "devel/ncurses/6.2-GCCcore-9.3.0".to_string(),
                    "devel/ncurses/6.2-GCCcore-10.2.0".to_string(),
                    "devel/ncurses/6.2".to_string(),
                    "devel/nsync/1.24.0-GCCcore-10.2.0".to_string(),
                    "devel/pkg-config/0.29.2-GCCcore-7.3.0".to_string(),
                    "devel/pkg-config/0.29.2-GCCcore-9.3.0".to_string(),
                    "devel/pkg-config/0.29.2-GCCcore-10.2.0".to_string(),
                    "devel/pkgconfig/1.5.1-GCCcore-9.3.0-Python-3.8.2".to_string(),
                    "devel/pkgconfig/1.5.1-GCCcore-10.2.0-python".to_string(),
                    "devel/protobuf/3.14.0-GCCcore-10.2.0".to_string(),
                    "devel/protobuf-python/3.14.0-GCCcore-10.2.0".to_string(),
                    "devel/sparsehash/2.0.4-GCCcore-10.2.0".to_string(),
                    "devel/typing-extensions/3.7.4.3-GCCcore-10.2.0".to_string(),
                    "devel/wget/1.20.3-GCCcore-10.2.0".to_string(),
                    "devel/xorg-macros/1.19.2-GCCcore-7.3.0".to_string(),
                    "devel/xorg-macros/1.19.2-GCCcore-9.3.0".to_string(),
                    "devel/xorg-macros/1.19.2-GCCcore-10.2.0".to_string(),
                ],
            },
            ModuleCategory {
                name: "lang".to_string(),
                modules: vec![
                    "compiler/Clang/11.0.1-gcccuda-2020b".to_string(),
                    "lang/Anaconda2/5.3.0".to_string(),
                    "lang/Anaconda3/2020.11".to_string(),
                    "lang/Anaconda3/2021.05".to_string(),
                    "lang/Bison/3.0.4-GCCcore-7.3.0".to_string(),
                    "lang/Bison/3.0.4".to_string(),
                    "lang/Bison/3.0.5-GCCcore-7.3.0".to_string(),
                    "lang/Bison/3.3.2-GCCcore-8.3.0".to_string(),
                    "lang/Bison/3.3.2".to_string(),
                    "lang/Bison/3.5.3-GCCcore-9.3.0".to_string(),
                    "lang/Bison/3.5.3".to_string(),
                    "lang/Bison/3.7.1-GCCcore-10.2.0".to_string(),
                    "lang/Bison/3.7.1".to_string(),
                    "lang/Cython/0.29.22-GCCcore-10.2.0".to_string(),
                    "lang/FriBidi/1.0.5-GCCcore-7.3.0".to_string(),
                    "lang/FriBidi/1.0.9-GCCcore-9.3.0".to_string(),
                    "lang/FriBidi/1.0.10-GCCcore-10.2.0".to_string(),
                    "lang/Guile/1.8.8-GCCcore-9.3.0".to_string(),
                    "lang/Guile/3.0.7-GCCcore-10.2.0".to_string(),
                    "lang/Java/1.8.0_281".to_string(),
                    "lang/Java/11.0.2".to_string(),
                    "lang/Julia/1.6.1-linux-x86_64".to_string(),
                    "lang/Julia/1.7.3-linux-x86_64".to_string(),
                    "lang/Lua/5.1.5-GCCcore-7.3.0".to_string(),
                    "lang/Lua/5.4.2-GCCcore-10.2.0".to_string(),
                    "lang/Miniconda2/4.7.10".to_string(),
                    "lang/Miniconda3/4.9.2".to_string(),
                    "lang/Mono/6.4.0.198-foss-2018b".to_string(),
                    "lang/NASM/2.13.03-GCCcore-7.3.0".to_string(),
                    "lang/NASM/2.14.02-GCCcore-9.3.0".to_string(),
                    "lang/NASM/2.15.05-GCCcore-10.2.0".to_string(),
                    "lang/Perl/5.28.0-GCCcore-7.3.0".to_string(),
                    "lang/Perl/5.30.2-GCCcore-9.3.0".to_string(),
                    "lang/Perl/5.32.0-GCCcore-10.2.0-minimal".to_string(),
                    "lang/Perl/5.32.0-GCCcore-10.2.0".to_string(),
                    "lang/Python/2.7.15-fosscuda-2018b".to_string(),
                    "lang/Python/2.7.15-GCCcore-7.3.0-bare".to_string(),
                    "lang/Python/2.7.18-GCCcore-9.3.0".to_string(),
                    "lang/Python/2.7.18-GCCcore-10.2.0".to_string(),
                    "lang/Python/3.8.2-GCCcore-9.3.0".to_string(),
                    "lang/Python/3.8.6-GCCcore-10.2.0".to_string(),
                    "lang/R/3.6.3-foss-2020a".to_string(),
                    "lang/R/4.0.0-foss-2020a".to_string(),
                    "lang/R/4.0.3-foss-2020b".to_string(),
                    "lang/R/4.0.3-fosscuda-2020b".to_string(),
                    "lang/R/4.0.4-foss-2020b".to_string(),
                    "lang/Ruby/2.7.2-GCCcore-10.2.0".to_string(),
                    "lang/Rust/1.42.0-GCCcore-9.3.0".to_string(),
                    "lang/SciPy-bundle/2020.03-foss-2020a-Python-3.8.2".to_string(),
                    "lang/SciPy-bundle/2020.11-foss-2020b-Python-2.7.18".to_string(),
                    "lang/SciPy-bundle/2020.11-foss-2020b".to_string(),
                    "lang/SciPy-bundle/2020.11-fosscuda-2020b".to_string(),
                    "lang/Tcl/8.6.8-GCCcore-7.3.0".to_string(),
                    "lang/Tcl/8.6.10-GCCcore-9.3.0".to_string(),
                    "lang/Tcl/8.6.10-GCCcore-10.2.0".to_string(),
                    "lang/Tkinter/2.7.18-GCCcore-10.2.0".to_string(),
                    "lang/Tkinter/3.8.2-GCCcore-9.3.0".to_string(),
                    "lang/Tkinter/3.8.6-GCCcore-10.2.0".to_string(),
                    "lang/Yasm/1.3.0-GCCcore-9.3.0".to_string(),
                    "lang/Yasm/1.3.0-GCCcore-10.2.0".to_string(),
                    "lang/dotNET-SDK/3.1.300-linux-x64".to_string(),
                    "lang/flex/2.6.4-GCCcore-7.3.0".to_string(),
                    "lang/flex/2.6.4-GCCcore-8.3.0".to_string(),
                    "lang/flex/2.6.4-GCCcore-9.3.0".to_string(),
                    "lang/flex/2.6.4-GCCcore-10.2.0".to_string(),
                    "lang/flex/2.6.4".to_string(),
                    "lang/nodejs/12.19.0-GCCcore-10.2.0".to_string(),
                    "lang/numba/0.52.0-foss-2020b".to_string(),
                    "lang/numba/0.52.0-fosscuda-2020b".to_string(),
                ],
            },
            ModuleCategory {
                name: "lib".to_string(),
                modules: vec![
                    "lib/alsa-lib/1.2.4-GCCcore-10.2.0".to_string(),
                    "lib/argtable/2.13-foss-2018b".to_string(),
                    "lib/double-conversion/3.1.5-GCCcore-9.3.0".to_string(),
                    "lib/double-conversion/3.1.5-GCCcore-10.2.0".to_string(),
                    "lib/gc/7.6.12-GCCcore-10.2.0".to_string(),
                    "lib/giflib/5.2.1-GCCcore-10.2.0".to_string(),
                    "lib/libaio/0.3.112-GCCcore-10.2.0".to_string(),
                    "lib/libdrm/2.4.92-GCCcore-7.3.0".to_string(),
                    "lib/libdrm/2.4.100-GCCcore-9.3.0".to_string(),
                    "lib/libepoxy/1.5.4-GCCcore-10.2.0".to_string(),
                    "lib/libevent/2.1.11-GCCcore-9.3.0".to_string(),
                    "lib/libevent/2.1.12-GCCcore-10.2.0".to_string(),
                    "lib/libfabric/1.11.0-GCCcore-9.3.0".to_string(),
                    "lib/libfabric/1.11.0-GCCcore-10.2.0".to_string(),
                    "lib/libffi/3.3-GCCcore-9.3.0".to_string(),
                    "lib/libgd/2.2.5-GCCcore-7.3.0".to_string(),
                    "lib/libgeotiff/1.6.0-GCCcore-10.2.0".to_string(),
                    "lib/libglvnd/1.2.0-GCCcore-9.3.0".to_string(),
                    "lib/libglvnd/1.3.2-GCCcore-10.2.0".to_string(),
                    "lib/libgpuarray/0.7.6-fosscuda-2020b".to_string(),
                    "lib/libharu/2.3.0-GCCcore-7.3.0".to_string(),
                    "lib/libiconv/1.15-GCCcore-7.3.0".to_string(),
                    "lib/libiconv/1.16-GCCcore-9.3.0".to_string(),
                    "lib/libiconv/1.16-GCCcore-10.2.0".to_string(),
                    "lib/libidn/1.36-GCCcore-10.2.0".to_string(),
                    "lib/libidn2/2.3.0-GCCcore-10.2.0".to_string(),
                    "lib/libjpeg-turbo/2.0.4-GCCcore-9.3.0".to_string(),
                    "lib/libjpeg-turbo/2.0.5-GCCcore-10.2.0".to_string(),
                    "lib/libmatheval/1.1.11-GCCcore-9.3.0".to_string(),
                    "lib/libogg/1.3.4-GCCcore-10.2.0".to_string(),
                    "lib/libpng/1.6.34-GCCcore-7.3.0".to_string(),
                    "lib/libpng/1.6.37-GCCcore-9.3.0".to_string(),
                    "lib/libpsl/0.21.1-GCCcore-10.2.0".to_string(),
                    "lib/libreadline/8.0-GCCcore-9.3.0".to_string(),
                    "lib/libreadline/8.0-GCCcore-10.2.0".to_string(),
                    "lib/libsndfile/1.0.28-GCCcore-10.2.0".to_string(),
                    "lib/libsodium/1.0.18-GCCcore-10.2.0".to_string(),
                    "lib/libspatialite/4.3.0a-foss-2020b-Python-3.8.6".to_string(),
                    "lib/libtasn1/4.16.0-GCCcore-10.2.0".to_string(),
                    "lib/libtirpc/1.3.1-GCCcore-10.2.0".to_string(),
                    "lib/libtool/2.4.6-GCCcore-7.3.0".to_string(),
                    "lib/libtool/2.4.6-GCCcore-9.3.0".to_string(),
                    "lib/libtool/2.4.6-GCCcore-10.2.0".to_string(),
                    "lib/libunistring/0.9.10-GCCcore-10.2.0".to_string(),
                    "lib/libunwind/1.2.1-GCCcore-7.3.0".to_string(),
                    "lib/libunwind/1.3.1-GCCcore-9.3.0".to_string(),
                    "lib/libvorbis/1.3.7-GCCcore-10.2.0".to_string(),
                    "lib/libwebp/1.1.0-GCCcore-10.2.0".to_string(),
                    "lib/libxml2/2.9.8-GCCcore-7.3.0".to_string(),
                    "lib/libxml2/2.9.10-GCCcore-9.3.0".to_string(),
                    "lib/libxml2/2.9.10-GCCcore-10.2.0".to_string(),
                    "lib/libxslt/1.1.34-GCCcore-9.3.0".to_string(),
                    "lib/libxslt/1.1.34-GCCcore-10.2.0".to_string(),
                    "lib/libyaml/0.2.2-GCCcore-9.3.0".to_string(),
                    "lib/libyaml/0.2.5-GCCcore-10.2.0".to_string(),
                    "lib/lxml/4.5.2-GCCcore-9.3.0".to_string(),
                    "lib/lz4/1.9.2-GCCcore-9.3.0".to_string(),
                    "lib/lz4/1.9.2-GCCcore-10.2.0".to_string(),
                    "lib/minizip/2.10.0-GCCcore-10.2.0".to_string(),
                    "lib/nettle/3.4-fosscuda-2018b".to_string(),
                    "lib/nettle/3.6-GCCcore-10.2.0".to_string(),
                    "lib/p11-kit/0.23.22-GCCcore-10.2.0".to_string(),
                    "lib/pocl/1.6-GCC-10.2.0".to_string(),
                    "lib/pocl/1.6-gcccuda-2020b".to_string(),
                    "lib/pybind11/2.4.3-GCCcore-9.3.0-Python-3.8.2".to_string(),
                    "lib/pybind11/2.6.0-GCCcore-10.2.0".to_string(),
                    "lib/scikit-build/0.11.1-fosscuda-2020b".to_string(),
                    "lib/snappy/1.1.8-GCCcore-9.3.0".to_string(),
                    "lib/snappy/1.1.8-GCCcore-10.2.0".to_string(),
                    "lib/tbb/2018_U5-GCCcore-7.3.0".to_string(),
                    "lib/tbb/2020.1-GCCcore-9.3.0".to_string(),
                    "lib/tbb/2020.3-GCCcore-10.2.0".to_string(),
                    "lib/zlib/1.2.11-GCCcore-8.3.0".to_string(),
                    "lib/zlib/1.2.11-GCCcore-9.3.0".to_string(),
                    "lib/zlib/1.2.11-GCCcore-10.2.0".to_string(),
                    "lib/zlib/1.2.11".to_string(),
                    "lib/zstd/1.4.4-GCCcore-9.3.0".to_string(),
                    "lib/zstd/1.4.5-GCCcore-10.2.0".to_string(),
                    "math/libcerf/1.7-GCCcore-7.3.0".to_string(),
                    "math/libcerf/1.14-GCCcore-10.2.0".to_string(),
                    "numlib/Armadillo/10.5.3-foss-2020b".to_string(),
                    "numlib/FFTW/3.3.8-gompi-2018b".to_string(),
                    "numlib/FFTW/3.3.8-gompi-2020b".to_string(),
                    "numlib/FFTW/3.3.8-gompic-2018b".to_string(),
                    "numlib/FFTW/3.3.8-gompic-2020b".to_string(),
                    "numlib/GSL/2.6-GCC-10.2.0".to_string(),
                    "numlib/LAPACK/3.9.1-GCC-10.2.0".to_string(),
                    "numlib/NLopt/2.6.1-GCCcore-9.3.0".to_string(),
                    "numlib/OpenBLAS/0.3.1-GCC-7.3.0-2.30".to_string(),
                    "numlib/OpenBLAS/0.3.9-GCC-9.3.0".to_string(),
                    "numlib/OpenBLAS/0.3.12-GCC-10.2.0".to_string(),
                    "numlib/ScaLAPACK/2.0.2-gompi-2018b-OpenBLAS-0.3.1".to_string(),
                    "numlib/ScaLAPACK/2.1.0-gompi-2020a".to_string(),
                    "numlib/ScaLAPACK/2.1.0-gompi-2020b".to_string(),
                    "numlib/ScaLAPACK/2.1.0-gompic-2020b".to_string(),
                    "numlib/SuiteSparse/5.8.1-foss-2020b-METIS-5.1.0".to_string(),
                    "numlib/beagle-lib/3.0.2-foss-2018b".to_string(),
                    "numlib/beagle-lib/3.1.2-GCC-10.2.0".to_string(),
                    "numlib/cuDNN/8.0.4.30-CUDA-11.1.1".to_string(),
                    "numlib/imkl/2020.4.304-gompi-2020b".to_string(),
                    "system/libgcrypt/1.9.2-GCCcore-10.2.0".to_string(),
                    "system/libpciaccess/0.14-GCCcore-7.3.0".to_string(),
                    "system/libpciaccess/0.16-GCCcore-9.3.0".to_string(),
                    "tools/libarchive/3.4.3-GCCcore-10.2.0".to_string(),
                    "vis/libGLU/9.0.1-GCCcore-9.3.0".to_string(),
                    "vis/libGLU/9.0.1-GCCcore-10.2.0".to_string(),
                    "vis/matplotlib/3.2.1-foss-2020a-Python-3.8.2".to_string(),
                    "vis/matplotlib/3.3.3-foss-2020b".to_string(),
                ],
            },
            ModuleCategory {
                name: "math".to_string(),
                modules: vec![
                    "math/Eigen/3.3.7-GCCcore-9.3.0".to_string(),
                    "math/Eigen/3.3.8-GCCcore-10.2.0".to_string(),
                    "math/GEOS/3.9.1-GCC-10.2.0".to_string(),
                    "math/GMP/6.2.0-GCCcore-9.3.0".to_string(),
                    "math/GMP/6.2.0-GCCcore-10.2.0".to_string(),
                    "math/ISL/0.23-GCCcore-10.2.0".to_string(),
                    "math/Keras/2.4.3-fosscuda-2020b".to_string(),
                    "math/MATLAB/2023b".to_string(),
                    "math/METIS/5.1.0-GCCcore-10.2.0".to_string(),
                    "math/MPC/1.2.1-GCCcore-10.2.0".to_string(),
                    "math/MPFR/4.1.0-GCCcore-10.2.0".to_string(),
                    "math/Theano/1.1.2-fosscuda-2020b-PyMC".to_string(),
                    "math/Voro++/0.4.6-GCCcore-9.3.0".to_string(),
                    "math/libcerf/1.7-GCCcore-7.3.0".to_string(),
                    "math/libcerf/1.14-GCCcore-10.2.0".to_string(),
                    "math/lpsolve/5.5.2.11-GCC-10.2.0".to_string(),
                    "math/magma/2.5.4-fosscuda-2020b".to_string(),
                    "math/molmod/1.4.5-foss-2020a-Python-3.8.2".to_string(),
                ],
            },
            ModuleCategory {
                name: "mpi".to_string(),
                modules: vec![
                    "mpi/MPICH/3.3.2-GCC-10.2.0".to_string(),
                    "mpi/OpenMPI/3.1.1-GCC-7.3.0-2.30".to_string(),
                    "mpi/OpenMPI/3.1.1-gcccuda-2018b".to_string(),
                    "mpi/OpenMPI/4.0.3-GCC-9.3.0".to_string(),
                    "mpi/OpenMPI/4.0.5-GCC-10.2.0".to_string(),
                    "mpi/OpenMPI/4.0.5-gcccuda-2020b".to_string(),
                    "mpi/OpenMPI/4.1.0-GCC-10.2.0".to_string(),
                    "numlib/FFTW/3.3.8-gompi-2018b".to_string(),
                    "numlib/FFTW/3.3.8-gompi-2020b".to_string(),
                    "numlib/FFTW/3.3.8-gompic-2018b".to_string(),
                    "numlib/FFTW/3.3.8-gompic-2020b".to_string(),
                    "numlib/ScaLAPACK/2.0.2-gompi-2018b-OpenBLAS-0.3.1".to_string(),
                    "numlib/ScaLAPACK/2.1.0-gompi-2020a".to_string(),
                    "numlib/ScaLAPACK/2.1.0-gompi-2020b".to_string(),
                    "numlib/ScaLAPACK/2.1.0-gompic-2020b".to_string(),
                    "numlib/imkl/2020.4.304-gompi-2020b".to_string(),
                    "toolchain/gompi/2018b".to_string(),
                    "toolchain/gompi/2020a".to_string(),
                    "toolchain/gompic/2018b".to_string(),
                    "toolchain/gompic/2020b".to_string(),
                    "tools/IOR/3.3.0-gompi-2020b".to_string(),
                    "tools/YAXT/0.9.0-gompi-2020b".to_string(),
                    "tools/ecCodes/2.20.0-gompi-2020b".to_string(),
                ],
            },
            ModuleCategory {
                name: "num".to_string(),
                modules: vec![
                    "numlib/Armadillo/10.5.3-foss-2020b".to_string(),
                    "numlib/FFTW/3.3.8-gompi-2018b".to_string(),
                    "numlib/FFTW/3.3.8-gompi-2020b".to_string(),
                    "numlib/FFTW/3.3.8-gompic-2018b".to_string(),
                    "numlib/FFTW/3.3.8-gompic-2020b".to_string(),
                    "numlib/GSL/2.6-GCC-10.2.0".to_string(),
                    "numlib/LAPACK/3.9.1-GCC-10.2.0".to_string(),
                    "numlib/NLopt/2.6.1-GCCcore-9.3.0".to_string(),
                    "numlib/OpenBLAS/0.3.1-GCC-7.3.0-2.30".to_string(),
                    "numlib/OpenBLAS/0.3.9-GCC-9.3.0".to_string(),
                    "numlib/OpenBLAS/0.3.12-GCC-10.2.0".to_string(),
                    "numlib/ScaLAPACK/2.0.2-gompi-2018b-OpenBLAS-0.3.1".to_string(),
                    "numlib/ScaLAPACK/2.1.0-gompi-2020a".to_string(),
                    "numlib/ScaLAPACK/2.1.0-gompi-2020b".to_string(),
                    "numlib/ScaLAPACK/2.1.0-gompic-2020b".to_string(),
                    "numlib/SuiteSparse/5.8.1-foss-2020b-METIS-5.1.0".to_string(),
                    "numlib/beagle-lib/3.0.2-foss-2018b".to_string(),
                    "numlib/beagle-lib/3.1.2-GCC-10.2.0".to_string(),
                    "numlib/cuDNN/8.0.4.30-CUDA-11.1.1".to_string(),
                    "numlib/imkl/2020.4.304-gompi-2020b".to_string(),
                ],
            },
            ModuleCategory {
                name: "system".to_string(),
                modules: vec![
                    "system/CUDA/8.0.61".to_string(),
                    "system/CUDA/11.1.1-GCC-10.2.0".to_string(),
                    "system/GnuTLS/3.7.1-GCC-10.2.0".to_string(),
                    "system/OpenPGM/5.2.122-GCCcore-9.3.0".to_string(),
                    "system/OpenPGM/5.2.122-GCCcore-10.2.0".to_string(),
                    "system/hwloc/1.11.10-GCCcore-7.3.0".to_string(),
                    "system/hwloc/2.2.0-GCCcore-9.3.0".to_string(),
                    "system/hwloc/2.2.0-GCCcore-10.2.0".to_string(),
                    "system/libgcrypt/1.9.2-GCCcore-10.2.0".to_string(),
                    "system/libpciaccess/0.14-GCCcore-7.3.0".to_string(),
                    "system/libpciaccess/0.16-GCCcore-9.3.0".to_string(),
                ],
            },
            ModuleCategory {
                name: "toolchain".to_string(),
                modules: vec![
                    "toolchain/foss/2018b".to_string(),
                    "toolchain/foss/2020a".to_string(),
                    "toolchain/fosscuda/2018b".to_string(),
                    "toolchain/fosscuda/2020b".to_string(),
                    "toolchain/gcccuda/2020b".to_string(),
                    "toolchain/gompi/2018b".to_string(),
                    "toolchain/gompi/2020a".to_string(),
                    "toolchain/gompic/2018b".to_string(),
                    "toolchain/gompic/2020b".to_string(),
                ],
            },
            ModuleCategory {
                name: "tool".to_string(),
                modules: vec![
                    "tools/DB/18.1.40-GCCcore-10.2.0".to_string(),
                    "tools/DMTCP/2.6.0-GCCcore-9.3.0".to_string(),
                    "tools/EasyBuild/4.5.0".to_string(),
                    "tools/GLPK/4.65-GCCcore-9.3.0".to_string(),
                    "tools/Ghostscript/9.52-GCCcore-9.3.0".to_string(),
                    "tools/Ghostscript/9.53.3-GCCcore-10.2.0".to_string(),
                    "tools/HPL/2.3-foss-2020b".to_string(),
                    "tools/IOR/3.3.0-gompi-2020b".to_string(),
                    "tools/IPython/7.15.0-foss-2020a-Python-3.8.2".to_string(),
                    "tools/IPython/7.18.1-GCCcore-10.2.0".to_string(),
                    "tools/IRkernel/1.1-foss-2020a-R-3.6.3-Python-3.8.2".to_string(),
                    "tools/ImageJ/1.51k".to_string(),
                    "tools/Meson/0.55.1-GCCcore-9.3.0-Python-3.8.2".to_string(),
                    "tools/Meson/0.55.3-GCCcore-10.2.0".to_string(),
                    "tools/Ninja/1.10.1-GCCcore-10.2.0".to_string(),
                    "tools/Pandoc/2.13".to_string(),
                    "tools/Szip/2.1.1-GCCcore-7.3.0".to_string(),
                    "tools/Szip/2.1.1-GCCcore-10.2.0".to_string(),
                    "tools/UnZip/6.0-GCCcore-10.2.0".to_string(),
                    "tools/VTune/2021.6.0".to_string(),
                    "tools/XZ/5.2.4-GCCcore-7.3.0".to_string(),
                    "tools/XZ/5.2.5-GCCcore-10.2.0".to_string(),
                    "tools/YAXT/0.9.0-gompi-2020b".to_string(),
                    "tools/Zip/3.0-GCCcore-10.2.0".to_string(),
                    "tools/archspec/0.1.0-GCCcore-9.3.0-Python-3.8.2".to_string(),
                    "tools/binutils/2.30-GCCcore-7.3.0".to_string(),
                    "tools/binutils/2.30".to_string(),
                    "tools/binutils/2.32-GCCcore-8.3.0".to_string(),
                    "tools/binutils/2.34-GCCcore-9.3.0".to_string(),
                    "tools/binutils/2.34".to_string(),
                    "tools/binutils/2.35-GCCcore-10.2.0".to_string(),
                    "tools/binutils/2.35".to_string(),
                    "tools/bokeh/2.2.3-foss-2020b".to_string(),
                    "tools/bokeh/2.2.3-fosscuda-2020b".to_string(),
                    "tools/bzip2/1.0.6-GCCcore-7.3.0".to_string(),
                    "tools/bzip2/1.0.8-GCCcore-10.2.0".to_string(),
                    "tools/cURL/7.60.0-GCCcore-7.3.0".to_string(),
                    "tools/cURL/7.72.0-GCCcore-10.2.0".to_string(),
                    "tools/ecCodes/2.20.0-gompi-2020b".to_string(),
                    "tools/expat/2.2.5-GCCcore-7.3.0".to_string(),
                    "tools/expat/2.2.9-GCCcore-9.3.0".to_string(),
                    "tools/expat/2.2.9-GCCcore-10.2.0".to_string(),
                    "tools/gettext/0.19.8.1-GCCcore-7.3.0".to_string(),
                    "tools/gettext/0.20.1-GCCcore-9.3.0".to_string(),
                    "tools/gettext/0.20.1".to_string(),
                    "tools/gettext/0.21-GCCcore-10.2.0".to_string(),
                    "tools/git/2.28.0-GCCcore-10.2.0-nodocs".to_string(),
                    "tools/groff/1.22.4-GCCcore-10.2.0".to_string(),
                    "tools/gzip/1.10-GCCcore-9.3.0".to_string(),
                    "tools/gzip/1.10-GCCcore-10.2.0".to_string(),
                    "tools/help2man/1.47.4-GCCcore-7.3.0".to_string(),
                    "tools/help2man/1.47.4".to_string(),
                    "tools/help2man/1.47.8-GCCcore-8.3.0".to_string(),
                    "tools/help2man/1.47.12-GCCcore-9.3.0".to_string(),
                    "tools/hypothesis/5.41.2-GCCcore-10.2.0".to_string(),
                    "tools/hypothesis/5.41.5-GCCcore-10.2.0".to_string(),
                    "tools/jq/1.6-GCCcore-10.2.0".to_string(),
                    "tools/lftp/4.9.2-GCC-10.2.0".to_string(),
                    "tools/libarchive/3.4.3-GCCcore-10.2.0".to_string(),
                    "tools/likwid/5.1.0-GCCcore-9.3.0".to_string(),
                    "tools/mdtest/1.9.3-foss-2020b".to_string(),
                    "tools/networkx/2.5-foss-2020b".to_string(),
                    "tools/networkx/2.5-fosscuda-2020b".to_string(),
                    "tools/numactl/2.0.11-GCCcore-7.3.0".to_string(),
                    "tools/numactl/2.0.13-GCCcore-10.2.0".to_string(),
                    "tools/oniguruma/6.9.7.1-GCCcore-10.2.0".to_string(),
                    "tools/poetry/1.0.9-GCCcore-9.3.0-Python-3.8.2".to_string(),
                    "tools/pytest-xdist/2.3.0-GCCcore-10.2.0".to_string(),
                    "tools/tcsh/6.20.00-GCCcore-7.3.0".to_string(),
                    "tools/umap-learn/0.4.6-fosscuda-2020b".to_string(),
                    "tools/util-linux/2.35-GCCcore-9.3.0".to_string(),
                    "tools/util-linux/2.36-GCCcore-10.2.0".to_string(),
                ],
            },
            ModuleCategory {
                name: "vis".to_string(),
                modules: vec![
                    "vis/ATK/2.36.0-GCCcore-10.2.0".to_string(),
                    "vis/ETE/3.1.2-foss-2020a-Python-3.8.2".to_string(),
                    "vis/ETE/3.1.2-foss-2020b".to_string(),
                    "vis/FFmpeg/4.3.1-GCCcore-10.2.0".to_string(),
                    "vis/FLTK/1.3.4-fosscuda-2018b".to_string(),
                    "vis/FLTK/1.3.5-GCCcore-10.2.0".to_string(),
                    "vis/GLib/2.54.3-GCCcore-7.3.0".to_string(),
                    "vis/GLib/2.64.1-GCCcore-9.3.0".to_string(),
                    "vis/GLib/2.66.1-GCCcore-10.2.0".to_string(),
                    "vis/GST-plugins-base/1.18.4-GCC-10.2.0".to_string(),
                    "vis/GStreamer/1.18.3-GCC-10.2.0".to_string(),
                    "vis/GTK+/3.24.23-GCCcore-10.2.0".to_string(),
                    "vis/Gdk-Pixbuf/2.40.0-GCCcore-10.2.0".to_string(),
                    "vis/HarfBuzz/2.2.0-fosscuda-2018b".to_string(),
                    "vis/HarfBuzz/2.6.7-GCCcore-10.2.0".to_string(),
                    "vis/ImageMagick/7.0.10-35-GCCcore-10.2.0".to_string(),
                    "vis/JasPer/2.0.14-GCCcore-9.3.0".to_string(),
                    "vis/JasPer/2.0.24-GCCcore-10.2.0".to_string(),
                    "vis/LittleCMS/2.9-GCCcore-9.3.0".to_string(),
                    "vis/LittleCMS/2.11-GCCcore-10.2.0".to_string(),
                    "vis/Mesa/20.0.2-GCCcore-9.3.0".to_string(),
                    "vis/Mesa/20.2.1-GCCcore-10.2.0".to_string(),
                    "vis/OpenEXR/2.5.5-GCCcore-10.2.0".to_string(),
                    "vis/Pango/1.42.4-fosscuda-2018b".to_string(),
                    "vis/Pillow/8.0.1-GCCcore-10.2.0".to_string(),
                    "vis/PyQt5/5.15.1-GCCcore-9.3.0-Python-3.8.2".to_string(),
                    "vis/PyQt5/5.15.1-GCCcore-10.2.0".to_string(),
                    "vis/SFML/2.5.1-Linux_x86_64".to_string(),
                    "vis/Tk/8.6.10-GCCcore-9.3.0".to_string(),
                    "vis/Tk/8.6.10-GCCcore-10.2.0".to_string(),
                    "vis/X11/20180604-GCCcore-7.3.0".to_string(),
                    "vis/X11/20200222-GCCcore-9.3.0".to_string(),
                    "vis/Xvfb/1.20.9-GCCcore-9.3.0".to_string(),
                    "vis/Xvfb/1.20.9-GCCcore-10.2.0".to_string(),
                    "vis/at-spi2-atk/2.38.0-GCCcore-10.2.0".to_string(),
                    "vis/at-spi2-core/2.38.0-GCCcore-10.2.0".to_string(),
                    "vis/cairo/1.14.12-GCCcore-7.3.0".to_string(),
                    "vis/cairo/1.16.0-GCCcore-9.3.0".to_string(),
                    "vis/cisTEM/1.0.0-beta".to_string(),
                    "vis/fontconfig/2.13.0-GCCcore-7.3.0".to_string(),
                    "vis/fontconfig/2.13.92-GCCcore-9.3.0".to_string(),
                    "vis/fontconfig/2.13.92-GCCcore-10.2.0".to_string(),
                    "vis/freetype/2.9.1-GCCcore-7.3.0".to_string(),
                    "vis/freetype/2.10.1-GCCcore-9.3.0".to_string(),
                    "vis/freetype/2.10.3-GCCcore-10.2.0".to_string(),
                    "vis/gnuplot/5.4.1-GCCcore-10.2.0".to_string(),
                    "vis/jbigkit/2.1-GCCcore-10.2.0".to_string(),
                    "vis/libGLU/9.0.1-GCCcore-9.3.0".to_string(),
                    "vis/libGLU/9.0.1-GCCcore-10.2.0".to_string(),
                    "vis/matplotlib/3.2.1-foss-2020a-Python-3.8.2".to_string(),
                    "vis/matplotlib/3.3.3-foss-2020b".to_string(),
                    "vis/pixman/0.34.0-GCCcore-7.3.0".to_string(),
                    "vis/pixman/0.38.4-GCCcore-9.3.0".to_string(),
                    "vis/scikit-image/0.18.1-foss-2020b".to_string(),
                    "vis/torchvision/0.8.2-fosscuda-2020b-PyTorch-1.7.1".to_string(),
                    "vis/x264/20191217-GCCcore-9.3.0".to_string(),
                    "vis/x264/20201026-GCCcore-10.2.0".to_string(),
                    "vis/x265/3.3-GCCcore-9.3.0".to_string(),
                    "vis/x265/3.3-GCCcore-10.2.0".to_string(),
                    "vis/xprop/1.2.5-GCCcore-10.2.0".to_string(),
                ],
            },
        ];
        let mut app = App {
            categories,
            category_state: ListState::default(),
            module_state: ListState::default(),
            input_field: 0,
            dropdown_open: false,
            running: true,
            ..Default::default()
        };
        app.category_state.select(Some(0));
        app.module_state.select(Some(0));
        app
    }

    fn generate_slurm_script(&self) -> String {
        format!(
            "#!/bin/bash\n\
             {}\n\
             #SBATCH --partition=all\n\
             #SBATCH --nodes={}\n\
             #SBATCH --ntasks={}\n\
             #SBATCH --cpus-per-task={}\n\
             #SBATCH --mem={}G\n\
             #SBATCH --time={}\n\
             #SBATCH --chdir={}\n\
             #SBATCH --mail-type=ALL\n\
             #SBATCH --output=slurm-%j.out\n\
             {}\n\
             export PATH={}:$PATH\n\
             module load {}\n\
             #{}",
            self.username,
            self.nodes,
            self.tasks,
            self.cpu,
            self.memory,
            self.time,
            self.workdir,
            self.command,
            self.exportpath,
            self.categories[self.selected_category].modules[self.selected_module],
            self.additionaltext
        )
    }

    fn save_slurm_script(&self) -> io::Result<()> {
        let mut file = File::create("slurm_job.sh")?;
        file.write_all(self.generate_slurm_script().as_bytes())?;
        Ok(())
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    let mut app = App::new();

    while app.running {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let header = Paragraph::new(vec![
                Line::from(Span::styled(
                    "Slurm Configurator",
                    Style::default().fg(Color::Yellow),
                )),
                Line::from(Span::raw("Developed by Gaurav Sablok")),
            ])
            .block(Block::default().borders(Borders::ALL).title("Header"));
            f.render_widget(header, chunks[0]);
            let main_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[1]);
            let inputs = vec![
                format!("Username: {}", app.username),
                format!("Nodes: {}", app.nodes),
                format!("Tasks: {}", app.tasks),
                format!("CPUs: {}", app.cpu),
                format!("Memory: {}", app.memory),
                format!("Time: {}", app.time),
                format!("Work Directory: {}", app.workdir),
                format!("Email: {}", app.email),
                format!("Command: {}", app.command),
                format!("Export Path: {}", app.exportpath),
                format!("Additional Text: {}", app.additionaltext),
            ];
            let input_items: Vec<ListItem> = inputs
                .iter()
                .enumerate()
                .map(|(i, input)| {
                    if i == app.input_field && app.input_field < 11 {
                        ListItem::new(input.as_str()).style(Style::default().fg(Color::Green))
                    } else {
                        ListItem::new(input.as_str())
                    }
                })
                .collect();
            let input_list = List::new(input_items)
                .block(Block::default().borders(Borders::ALL).title("Inputs"));
            f.render_widget(input_list, main_chunks[0]);
            let module_chunks = if app.dropdown_open {
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                    .split(main_chunks[1])
            } else {
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(main_chunks[1])
            };

            let category_items: Vec<ListItem> = app
                .categories
                .iter()
                .enumerate()
                .map(|(i, cat)| {
                    if i == app.selected_category && app.input_field == 11 {
                        ListItem::new(cat.name.as_str()).style(Style::default().fg(Color::Yellow))
                    } else {
                        ListItem::new(cat.name.as_str())
                    }
                })
                .collect();

            let category_list = List::new(category_items)
                .block(Block::default().borders(Borders::ALL).title(format!(
                    "Category: {} {}",
                    app.categories[app.selected_category].name,
                    if app.dropdown_open { "▼" } else { "▶" }
                )));

            if app.input_field == 11 {
                f.render_stateful_widget(category_list, module_chunks[0], &mut app.category_state);
            } else {
                f.render_widget(category_list, module_chunks[0]);
            }

            if app.dropdown_open {
                let module_items: Vec<ListItem> = app.categories[app.selected_category]
                    .modules
                    .iter()
                    .enumerate()
                    .map(|(i, m)| {
                        if i == app.selected_module && app.input_field == 11 {
                            ListItem::new(m.as_str()).style(Style::default().fg(Color::Green))
                        } else {
                            ListItem::new(m.as_str())
                        }
                    })
                    .collect();

                let module_list = List::new(module_items)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Select Module ↓"),
                    )
                    .highlight_style(Style::default().fg(Color::Green));

                f.render_stateful_widget(module_list, module_chunks[1], &mut app.module_state);
            } else {
                let selected_module_display = format!(
                    "Selected: {}",
                    app.categories[app.selected_category].modules[app.selected_module]
                );
                let selected_para = Paragraph::new(selected_module_display).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Selected Module"),
                );
                f.render_widget(selected_para, module_chunks[0]);
            }
            let instructions = if app.input_field == 11 {
                if app.dropdown_open {
                    "↑↓: Navigate modules | Enter: Select module | Esc: Close dropdown | Tab: Switch to inputs"
                } else {
                    "↑↓: Navigate categories | Enter: Open module dropdown | Tab: Switch to inputs | q: Quit"
                }
            } else {
                "Tab: Switch fields | Enter: Submit | Space: Edit category | q: Quit"
            };

            let footer = Paragraph::new(instructions)
                .block(Block::default().borders(Borders::ALL).title("Instructions"));
            f.render_widget(footer, chunks[2]);
        })?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    app.running = false;
                }
                KeyCode::Tab => {
                    app.input_field = (app.input_field + 1) % 12;
                    if app.input_field == 11 {
                        app.dropdown_open = false;
                    }
                }
                KeyCode::Char(' ') => {
                    if app.input_field < 11 {
                        app.input_field = 11;
                        app.dropdown_open = false;
                    }
                }
                KeyCode::Esc => {
                    app.dropdown_open = false;
                }
                KeyCode::Up => {
                    if app.input_field == 11 {
                        if app.dropdown_open {
                            let selected = app.module_state.selected().unwrap_or(0);
                            if selected > 0 {
                                app.module_state.select(Some(selected - 1));
                                app.selected_module = selected - 1;
                            }
                        } else {
                            let selected = app.category_state.selected().unwrap_or(0);
                            if selected > 0 {
                                app.category_state.select(Some(selected - 1));
                                app.selected_category = selected - 1;
                                app.module_state.select(Some(0));
                                app.selected_module = 0;
                            }
                        }
                    }
                }
                KeyCode::Down => {
                    if app.input_field == 11 {
                        if app.dropdown_open {
                            let selected = app.module_state.selected().unwrap_or(0);
                            let max_modules = app.categories[app.selected_category].modules.len();
                            if selected < max_modules.saturating_sub(1) {
                                app.module_state.select(Some(selected + 1));
                                app.selected_module = selected + 1;
                            }
                        } else {
                            let selected = app.category_state.selected().unwrap_or(0);
                            if selected < app.categories.len().saturating_sub(1) {
                                app.category_state.select(Some(selected + 1));
                                app.selected_category = selected + 1;
                                app.module_state.select(Some(0));
                                app.selected_module = 0;
                            }
                        }
                    }
                }
                KeyCode::Enter => {
                    if app.input_field == 11 {
                        if app.dropdown_open {
                            app.dropdown_open = false;
                        } else {
                            app.dropdown_open = true;
                            app.module_state.select(Some(0));
                            app.selected_module = 0;
                        }
                    } else {
                        app.save_slurm_script()?;
                        println!("{}", app.generate_slurm_script());
                        app.running = false;
                    }
                }
                KeyCode::Char(c) => {
                    if app.input_field < 11 {
                        match app.input_field {
                            0 => app.username.push(c),
                            1 => app.nodes.push(c),
                            2 => app.tasks.push(c),
                            3 => app.cpu.push(c),
                            4 => app.memory.push(c),
                            5 => app.time.push(c),
                            6 => app.workdir.push(c),
                            7 => app.email.push(c),
                            8 => app.command.push(c),
                            9 => app.exportpath.push(c),
                            10 => app.additionaltext.push(c),
                            _ => {}
                        }
                    }
                }
                KeyCode::Backspace => match app.input_field {
                    0 => {
                        app.username.pop();
                    }
                    1 => {
                        app.nodes.pop();
                    }
                    2 => {
                        app.tasks.pop();
                    }
                    3 => {
                        app.cpu.pop();
                    }
                    4 => {
                        app.memory.pop();
                    }
                    5 => {
                        app.time.pop();
                    }
                    6 => {
                        app.workdir.pop();
                    }
                    7 => {
                        app.email.pop();
                    }
                    8 => {
                        app.command.pop();
                    }
                    9 => {
                        app.exportpath.pop();
                    }
                    10 => {
                        app.additionaltext.pop();
                    }
                    _ => {}
                },
                KeyCode::Enter => {
                    if app.dropdown_open {
                        // Select module and close dropdown
                        app.dropdown_open = false;
                    } else {
                        // Save the Slurm script
                        app.save_slurm_script()?;
                        println!("{}", app.generate_slurm_script());
                        app.running = false;
                    }
                }
                _ => {}
            }
        }
    }
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
