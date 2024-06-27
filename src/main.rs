use masterbundle_collector::masterbundle::MasterBundle;

fn main() -> anyhow::Result<()> {
    let bundle = MasterBundle::new(
        r"C:\Program Files (x86)\Steam\steamapps\common\U3DS\Servers\Escalation\Workshop\Steam\content\304930\3251926587\Escalation\Bundles\MasterBundle.dat".into(),
    )?;
    let parse = bundle.parse()?;

    println!("{:#?}", parse);

    Ok(())
}
