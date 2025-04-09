use rspack_cacheable::{cacheable, cacheable_dyn};
use rspack_util::ext::DynHash;

use crate::{
  Compilation, DependencyTemplate, DynamicDependencyTemplate, DynamicDependencyTemplateType,
  RuntimeGlobals, RuntimeSpec, TemplateContext, TemplateReplaceSource,
};

#[cacheable]
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct RuntimeRequirementsDependency {
  pub runtime_requirements: RuntimeGlobals,
}

#[cacheable_dyn]
impl DependencyTemplate for RuntimeRequirementsDependency {
  fn dynamic_dependency_template(&self) -> Option<DynamicDependencyTemplateType> {
    Some(RuntimeRequirementsDependencyTemplate::template_type())
  }

  fn update_hash(
    &self,
    hasher: &mut dyn std::hash::Hasher,
    _compilation: &Compilation,
    _runtime: Option<&RuntimeSpec>,
  ) {
    self.runtime_requirements.dyn_hash(hasher);
  }
}

impl RuntimeRequirementsDependency {
  pub fn new(runtime_requirements: RuntimeGlobals) -> Self {
    Self {
      runtime_requirements,
    }
  }
}

#[cacheable]
#[derive(Debug, Clone, Default)]
pub struct RuntimeRequirementsDependencyTemplate;

impl RuntimeRequirementsDependencyTemplate {
  pub fn template_type() -> DynamicDependencyTemplateType {
    DynamicDependencyTemplateType::CustomType("RuntimeRequirementsDependency")
  }
}

impl DynamicDependencyTemplate for RuntimeRequirementsDependencyTemplate {
  fn render(
    &self,
    dep: &dyn DependencyTemplate,
    _source: &mut TemplateReplaceSource,
    code_generatable_context: &mut TemplateContext,
  ) {
    let dep = dep
      .as_any()
      .downcast_ref::<RuntimeRequirementsDependency>()
      .expect(
        "RuntimeRequirementsDependencyTemplate should be used for RuntimeRequirementsDependency",
      );

    code_generatable_context
      .runtime_requirements
      .insert(dep.runtime_requirements);
  }
}
